fn main() {
    let nums: Vec<i32> = Vec::new();
    let mut nums = vec![1,2,3];

    nums.push(4);

    nums.remove(2);

    for num in &nums {
        println!("{}",num);
    }

    enum Value {
        Int(i32),
        Float(f32)
    };

    let random = vec![Value::Int(3), Value::Float(4.56)];

    for val in &random {
        println!("{}", val);
    } 
    
    //This should return an error because Value::Int 
    // and Value::Float are custom types without 
    // standard formatting IO methods
}
