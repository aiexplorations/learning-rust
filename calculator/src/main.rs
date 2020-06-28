// Coding up the calculator example from this helpful video - superb playlist by engineer man!
// https://youtu.be/RYTMn_kLItw 

use std::io::{ stdin, stdout, Write};

fn main() {
    println!("Welcome to the calculator!");
    println!("--------------");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();
    
    loop {
        println!("What is the first number?");
        read(&mut num1);

        println!("What is the second number?");
        read(&mut num2);

        println!("What operation would you like to do [ + - / * ]?");
        read(&mut operator);

        
        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        println!("{} {} {}", num1, num2, operator);


        let ops = String::from("+-*/");
        if !ops.contains(operator) {
            println!("Unknown operator!");
            continue;
        }

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("Error in operator")
        };

        println!("Result of {} {} {} = {}", num1, operator, num2, result);
    }

}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read line");
}

// Program generates occasional errors  as shown below

/*

What is the first number?
5
What is the second number?
6
What operation would you like to do [ + - / * ]?
/
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }', src\main.rs:25:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\calculator.exe` (exit code: 101)



*/