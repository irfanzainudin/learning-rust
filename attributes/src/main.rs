// https://medium.com/@luishrsoares/exploring-rust-attributes-in-depth-ac172993d568

#[cfg(target_os = "linux")]
fn main() {
    println!("Hello, Linus!");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("Hello, Steve!");
}
