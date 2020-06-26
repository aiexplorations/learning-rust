use std::io::Read;
use std::io;

fn main() {
    enum fruitKind {
        Apple,
        Orange,
    }

    println!("Enter your favourite fruit: ");
    let mut fruit = String::new();
    io::stdin().read_line(&mut fruit);
    let favFruit = match fruit {
        Apple => fruitKind::Apple,
        Orange => fruitKind::Orange,
    };
    println!("Your favourite fruit is: {}", favFruit);

}

/*

How to solve the error you get when running this?

How does one create non-standard output when printing an enum?

*/
