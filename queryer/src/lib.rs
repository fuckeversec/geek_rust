use anyhow::{anyhow, Result};
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};
use tracing::info;
mod convert;
mod dialect;
mod fetcher;
mod loader;
use convert::Sql;
pub use dialect::example_sql;
pub use dialect::TyrDialect;
use fetcher::retrieve_data;
use loader::detect_content;
#[derive(Debug)]
pub struct DataSet(DataFrame);

// 让DataSet用起来和DataFrame一致
impl Deref for DataSet {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 让DataSet用起来和DataFrame一致
impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 实现DataSet自己的函数
impl DataSet {
    pub fn to_csv(&self) -> Result<String> {
        let mut buf = Vec::new();
        let writer = CsvWriter::new(&mut buf);
        writer.finish(&self)?;
        Ok(String::from_utf8(buf)?)
    }
}

pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    // 整个SQL AST转换成我们定义的sql结构的细节都埋藏try_into()中
    // 只需要关注数据结构的使用, 怎么转换可以之后需要的时候才关注
    // 关注点分离
    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    info!("retrieving data from source: {}", source);

    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX));
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
