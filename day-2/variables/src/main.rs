fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    tuple_datatype();
    array_datatype();
    another_function();
}

// Tuple 
fn tuple_datatype() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

// Array
// Array in fixed length in Rust
fn array_datatype() {
    let a = [3; 5];
    // same as this
    let a = [3, 3, 3, 3, 3];
}

fn another_function() {
    println!("Another function.");
}

// functions 
fn function_example() {
    let y = 6;
}

// This is not allowed in Rust 
// In C, might be able to do something like this: x = y = 6
// I doubt you can do that in JavaScript either, anyway it looks like a bad practice in the first place.
fn function_example2() {
    let x = (let y = 6);
}

// Check out this example, the main has an express which is everything in the outer {}
// The x + 1 line doesn't have a semicolons because it is an expression.
// Expressions don't end with semicolons, and expressions do return value and statement don't.
// Keep in this mind.
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
}

// Function with return values

fn five() -> i32 {
    // no function calls, macros or even let statements within the function
    // just a number 5 by itself. It is perfectly valid.
    // return type as -> i32
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
}

// Okay, here is a overview of the code, the 5 in five is the return value, which why is typed i32.


// Take a look at this one
// Compiling this code produces an error suggesting removing the semicolons, given things we've learned today, think about why?
// There will be mismatchd types.
// Adding wa semicolons at the end of x + 1 turning it into a statement, no value, contradicting the function definition.
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}