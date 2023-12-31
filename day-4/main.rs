fn main() {
  let mut count = 0;

  let result = loop {
      counter += 1;

      if counter == 10 {
          break counter * 2;
      }
  };

  println!("The result is {result}");
}

// if there is a loop within a loop, break and continue apply to the 
// innermost loop, loop label is available to control specific loop.
// label must begin with a single quote ', that's like in JavaScript putting ref: in front of a loop.

fn main() {
  let mut count = 0;
  'counting_up: loop {
      println!("count = {count}");
      let mut remaining = 10;

      loop {
          println!("remaining = {remaining}")
          if remaining == 9 {
              break;
          }
          if count == 2 {
              break 'counting_up;
          }
          remaining -= 1;
      }

      count += 1;
  }
  println!("End count == {count}");
}

// While
fn main() {
  let mut number = 3;

  while number != 0 {
      println!("{number}");
      number -= 1;
  }

  println!("ha")
}

fn main() {
  let a = [1,2,3,4,5];
  let mut index = 0;

  while index < 5 {
      println!("the value is: {}", a[index]);

      index += 1;
  }

}

// For
fn main() {
  let a = [1,2,3,4,5];

  for ele in a {
      println!("the value is: {ele}");
  }
}