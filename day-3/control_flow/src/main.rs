fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// if condition has to be bool
// error out because if number isn't a bool format
// unlike JavaScript, Rust won't auto convert non-boolean to boolean.
// if has to be in a condition, keep that in mind.
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

// So the right way to write the above code would be
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

// using if in a let
// cause if is an expression, let is allowed to use at the left.
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// guess what happens here, Rust doesn't allow different types within a same condition.
fn main() {
    let condition = true;
    let number = if condition {5 } else { "five" };
    let number = if condition { 5 } else { "six" };
    
    println!("The value of number is: {number}");
}

// this is a loop, runs till it breaks.
fn main() {
    loop {
        println!("again!");
    }
}