fn main() {
    let x: i32 = 5;
    let y: Option<i32> = Some(3);

    let result = match type_of(x) == type_of(y) {
        Ok(value) => value,
        Err(error) => {
            panic!("Trouble adding numbers", error)
        },
    };
    
    //println!("{}", x + y).expect("Failed to add dissimilar types"); // Cannot add i32 to Option<i32>
    println!("{}", result);

    let z = y as i32;

    println!("{}", z + x);

}
