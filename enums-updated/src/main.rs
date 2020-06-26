use ::std::{
    io::{self,
        // Read,
    },
};

fn main ()
{
    #[derive(Debug)] // allows using {:?} formatting
    enum FruitKind {
        Apple,
        Orange,
    }

    println!("Enter your favourite fruit: ");

    let mut fruit = String::new();

    io::stdin().read_line(&mut fruit).unwrap();

    let fav_fruit = match fruit.as_str() {
        "apple" => FruitKind::Apple,
        "orange" => FruitKind::Orange,
        // Otherwise
        _ => {
            println!("Error, that fruit is not recognized");
            return;
        },
    };

    println!("Your favourite fruit is: {:?}", fav_fruit);
}

/*

Updated version thanks to the solution from Yandros:

https://users.rust-lang.org/t/correct-use-of-enums-in-a-match/44947/2

*/