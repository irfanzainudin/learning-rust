#[cfg(target_os = "linux")]
fn main() {
    println!("Hello, Linus!");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("Hello, Steve!");
}
