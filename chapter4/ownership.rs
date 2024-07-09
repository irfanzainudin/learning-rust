fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    println!("x after increment out of makes_copy fn: {x}"); // x remains 5 because when passing x into makes_copy, Rust copies the data entirely (it's a cheap operation to perform because i32 is stored on the stack). More info: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(mut some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
    some_integer += 1;
    println!("x after increment in makes_copy fn: {some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.