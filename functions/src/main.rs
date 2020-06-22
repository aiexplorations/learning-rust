fn main() {
    let x: i32 = 5;
    print_number(x);
    let x1: i32 = 10;
    let x2: i32 = 11;
    print_sum(x1, x2);

    print_number(add_one(x1));
}

fn print_number(x: i32){
    println!("The number is {}",x)
}

fn print_sum(x: i32, y: i32){
    print_number(x+y)
}

fn add_one(x: i32) -> i32 {
    x+1 // using a semicolon here returns an error
}