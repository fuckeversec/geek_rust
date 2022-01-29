pub fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    println!("s = {:p}", s);
    println!("*s = {:p}", *s);

    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // delimiter可能是utf8, 获取其utf8长度
        let suffix = &s[i + delimiter.len_utf8()..];
        println!("suffix = {:p}", suffix);
        // 将s指向剩余token
        *s = suffix;
        println!("s = {:p}, s = {}", s, s);
        println!("*s = {:p}, s = {}", *s, s);
        // 返回分割的第一个token
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    println!("s1 line 1 = {:p}", s1);

    let hello = strtok(&mut s1, ' ');
    println!("s1 line 2 = {:p}", s1);

    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}
