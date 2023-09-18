use std::io;

fn main() {
    println!("Input:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut expression = input;
    let operator = match expression.find('+') {
        Some(num) => num,
        None => 0,
    };
    let first: u32 = match expression[0..operator].parse() {
        Ok(num) => num,
        Err(_) => u32::MIN,
    };
    let second: u32 = match expression[operator..expression.len()].parse() {
        Ok(num) => num,
        Err(_) => u32::MIN,
    };

    println!("{}", first + second);

    // let mut expression = input;
    // while expression.contains('+')
    //     || expression.contains('-')
    //     || expression.contains('*')
    //     || expression.contains('/')
    // {
    //     let operator = match expression.find('+') {
    //         Some(num) => num,
    //         None => break,
    //     };
    //     let first: u32 = match expression[0..operator].parse() {
    //         Ok(num) => num,
    //         Err(_) => u32::MIN,
    //     };
    //     let second: u32 = match expression[operator..expression.len()].parse() {
    //         Ok(num) => num,
    //         Err(_) => u32::MIN,
    //     };
    //     let mut next = operator as u32;
    //     expression = expression.chars(next);
    // }

    // if input.contains('+') {
    //     let mut v: Vec<&str> = input.split('+').collect();
    //     for i in 0..v.len() {
    //         v[i] = v[i].trim();
    //     }
    //     let first: u32 = match v[0].parse() {
    //         Ok(num) => num,
    //         Err(_) => u32::MIN,
    //     };
    //     let second: u32 = match v[v.len() - 1].parse() {
    //         Ok(num) => num,
    //         Err(_) => u32::MIN,
    //     };
    //     let result = first + second;
    //     println!("Result:");
    //     println!("{}", &result);
    // }
}
