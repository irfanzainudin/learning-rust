# Notes from reading the [book](https://doc.rust-lang.org/book/)

What is high-level ergonomics?

"Rust is an *ahead-of-time* (AOT) compiled language" as compared to Javascript which uses *just-in-time* (JIT) compilation, if I remember correctly. Now I'm wondering if C uses AOT compilation. This [Reddit post](https://www.reddit.com/r/Compilers/comments/19ctf7p/aot_vs_jit_comilation/) seems to say that C is using AOT.

Also I found an interesting paper while searching for "ahead of time vs just in time compilation": [paper](https://dl.acm.org/doi/10.1145/3623507.3623554)

This is so slow and painful!!!

enum (or enumeration for long) is a type that can be in one of multiple possible states. These states are called variants. For example, Result has Ok(T) and Err(E).

Cute: "think of {} as little crab pincers that hold a value in place"

![Keywords Reserved by Rust for Future Use](imgs_for_notes/Screenshot%202024-07-03%20at%2012.07.27 PM.png) I can see some functional programming influences like `yield` in Haskell. `virtual` is interesting and new to me.

I just love the fact that variables in Rust are immutable by default. I don't know how to explain it, but it just makes sense in terms of safety to do so. And Rust advertises itself as a safe language. So for the price of annoyance for having to type in a few extra characters, we get safety.

Rust does not allow us to mutate a variable's type: maybe for safety reasons. Makes the lang easier to reason about.

Rust's signed numbers are stored using [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation.

*destructuring* is when you Rust breaks a single into multiple parts. Then we can access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

![A difference between C and Rust about statements and expressions](imgs_for_notes/Screenshot%202024-07-03%20at%202.22.34 PM.png) A difference between C and Rust

