fn main() {
    
    let func: fn(i32) -> i32 = plus_one; // This is a function pointer

    /*
    Having a function pointer enables us to point 
    to a specific function (as long as the return types match)
    and thereby instantiate variables based on the function pointer
    */

    let seven = func(6);

    println!("{}", seven);
}

fn plus_one(i: i32) -> i32 {
    i + 1
}

