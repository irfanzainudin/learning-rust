#[allow(unconditional_panic)]

// without the attribute above, this code won't compile
fn main() {
    let l = [1, 2, 3];
    l[6];
}
