use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().len() < 2 {
        println!("not set url");
        return Ok(());
    }

    for url in std::env::args().skip(1) {
        let output = "rust.md";
        println!("Fetching url: {}", url);

        let body = reqwest::blocking::get(url)?.text()?;
        println!("Converting html to markdown...");

        let md = html2md::parse_html(&body);

        fs::write(output, md.as_bytes())?;
        println!("Converted markdown has been saved in {}.", output);
    }
    Ok(())
}
