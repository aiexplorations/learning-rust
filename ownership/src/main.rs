fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("{}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


/*

This program illustrates how Rust handles objects and copies of them 
in the context of a specific scope. When we use these objects in a specific scope as "String::from"
we initialize an object in scope, and when this is passed to a function, the object stops being available
within the scope.

However, this is not the case with x, because we're initializing this directly
When passing x to a function we realize that it is copied and passed on.alloc

The string s is not available after we pass it to a function within the original scope. 
But the int x is available for other operations after being passed to makes_copy() 

Output:

hello
5  // this is produced by the function makes_copy()
5  // this is produced by the println! macro


When we use clone() on a variable, we make a copy of it and make it available 
for use without losing ownership

*/