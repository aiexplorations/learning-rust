fn main() {
    let s1 = String::from("hello"); // We're defining this string as before

    let len = calculate_length(&s1); // Here we pass the pointer of s1, rather than the value itself

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // s here is a pointer to a string, and not a string itself. 
            // Return type here is a usize, which is a pointer-sized unsigned integer type. 
            // Therefore the returned item is a pointer to the 
}
