use url::{Url, ParseError};

fn main() {
    let pathstring = String::from("http://google.com");
    let newstring = getPath(&pathstring).to_string();
    println!("Path String: {}", pathstring);
}

fn getPath(url: &str) -> &str{
    match Url::parse(&url) {
        Ok(parsed) => parsed.path(),
        Err(_err) => ""
    }
}
