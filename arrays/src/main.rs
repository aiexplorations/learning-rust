fn main() {
    let a = [1,2,3]; 
    let m = (1, [1,2,3]);
    println!("{:?}", a);
    println!("{:?}", m);
    println!("a has {} elements",a.len()) ;
    for item in a.iter() {
        if item % 2 == 0 {
            continue;
        }
        println!("{}", item);

        // Look up `outer and `inner in case of nested for loops
        // You can provide loop labels and control the looping this way
        // https://doc.rust-lang.org/1.17.0/book/loops.html#loop-labels 
    }
}