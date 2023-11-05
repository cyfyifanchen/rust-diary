fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

// Tuple 
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

// Array
// Array in fixed length in Rust
fn main() {
    let a = [3; 5];
    // same as this
    let a = [3, 3, 3, 3, 3];
}