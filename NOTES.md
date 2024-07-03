# Notes from reading the [book](https://doc.rust-lang.org/book/)

What is high-level ergonomics?

"Rust is an *ahead-of-time* (AOT) compiled language" as compared to Javascript which uses *just-in-time* (JIT) compilation, if I remember correctly. Now I'm wondering if C uses AOT compilation. This [Reddit post](https://www.reddit.com/r/Compilers/comments/19ctf7p/aot_vs_jit_comilation/) seems to say that C is using AOT.

Also I found an interesting paper while searching for "ahead of time vs just in time compilation": [paper](https://dl.acm.org/doi/10.1145/3623507.3623554)

This is so slow and painful!!!

enum (or enumeration for long) is a type that can be in one of multiple possible states. These states are called variants. For example, Result has Ok(T) and Err(E).

Cute: "think of {} as little crab pincers that hold a value in place"

![Keywords Reserved by Rust for Future Use](imgs_for_notes/Screenshot%202024-07-03%20at%2012.07.27 PM.png) I can see some functional programming influences like `yield` in Haskell. `virtual` is interesting and new to me.