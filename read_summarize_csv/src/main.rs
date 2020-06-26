extern crate csv;

use std::process;
use std::io;


fn main() {
    let mut reader = csv::Reader::from_path("mtcars.csv").unwrap();
    for line in reader.records() {
        
        match line {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("Error reading csv from stdin: {}", err),
                process::exit(1),
            }
        }
        
    }

}
