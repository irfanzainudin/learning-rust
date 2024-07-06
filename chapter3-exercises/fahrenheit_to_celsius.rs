// Formula:
// (32°F − 32) × 5/9 = 0°C
// (50°F − 32) × 5/9 = 10°C
// (60.8°F − 32) × 5/9 = 16°C
// (68°F − 32) × 5/9 = 20°C
// (86°F − 32) × 5/9 = 30°C
// (104°F − 32) × 5/9 = 40°C
// (122°F − 32) × 5/9 = 50°C

use std::io;
use std::io::Write;

fn main() {
    print!("Converting Fahrenheit to Celsius: ");
    let _ = io::stdout().flush();

    let mut t_in_f: String = String::new();

    io::stdin()
        .read_line(&mut t_in_f)
        .expect("Failed to read line");

    let t_in_c: f32 = match t_in_f.trim().parse::<f32>() {
        Ok(num) => ((num - 32.0) * 5.0)/9.0,
        Err(_) => 0.0,
    };

    println!("{t_in_c}°C");

    print!("Converting Celsius to Fahrenheit: ");
    let _ = io::stdout().flush();

    let mut t_in_c: String = String::new();

    io::stdin()
        .read_line(&mut t_in_c)
        .expect("Failed to read line");

    let t_in_f: f32 = match t_in_c.trim().parse::<f32>() {
        Ok(num) => ((num * 9.0)/5.0) + 32.0,
        Err(_) => 0.0,
    };

    println!("{t_in_f}°F");
}