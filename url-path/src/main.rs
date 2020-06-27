use url::{Url, ParseError};

fn main() {
    let pathstring = String::from("http://google.com");
    let newstring = getPath(&pathstring).to_string();
    println!("Path String: {}", pathstring);
}

fn getPath(url: &str) -> String{ // Have to convert this &str to String, and return strings using to_string() method
    match Url::parse(&url) {
        Ok(parsed) => parsed.path().to_string(),
        Err(_err) => "".to_string(),
    }
}


// Solution to the &str -> String parsing trick obtained from https://users.rust-lang.org/t/borrowing-problem-in-function-to-get-path-of-an-url/44946/10?u=aiexplorations 
