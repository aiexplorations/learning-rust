use std::io;

fn main() {
    let v = vec![1,2,3,4,5];
    let u = vec![0.0; 100];
    println!("Number of elements in u is {}", u.len());
    println!("3rd element of v is {}", v[2]);

    let mut index = String::new();
    println!("Enter the index to display: ");
    io::stdin().read_line(&mut index)
               .expect("Failed to read line");

    // Needed the complicated looking code below because of Rust's strict type system
    // We essentially cast index as a usize type, handle errors in the process, and return the usize variable index.
    // We then create a separate match expression to index our vector by this, and then display what we need to
    
    let index = match usize::from_str_radix(index.trim(),10) {
        Ok(v) => v,
        Err(err) => {eprintln!("bad index : {}", err);
        return;
    },
    };

    match v.get(index) {
        Some(x) => println!("Item at index {} is {}", index, x),
        None => println!("Sorry, this vector does not have that index.")
    }
}
