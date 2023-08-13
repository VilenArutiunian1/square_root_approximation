extern crate rug;

use std::io;
use std::pin::Pin;
use rug::Float;

fn main() {
    println!("Enter the number: ");
    let stdin = io::stdin();
    let mut number = String::new();
    stdin.read_line(&mut number).unwrap();
    let number = number.trim().parse::<usize>().unwrap();
    println!("Enter the digits: ");
    let mut digits = String::new();
    stdin.read_line(&mut digits).unwrap();
    let digits = digits.trim().parse::<u32>().unwrap();
    let bits = ((digits + 1) as f64 * std::f64::consts::LOG2_10).ceil() as u32;
    println!("It takes {} bits to store {} digits", bits, digits);
    let mut sqrt = Float::with_val(bits, number).sqrt();
    Pin::new(&mut sqrt).get_mut().set_prec(digits * 4); // 4 bits are required to encode a digit from 0-9
    let sqrt_string = sqrt.to_string();
    let mut parts = sqrt_string.split('.');
    let int_part = parts.next().unwrap();
    let decimal_part = parts.next().unwrap();
    let decimal_part_trimmed = &decimal_part[..digits as usize]; 
    println!("Sqrt of {}: {}.{}", number, int_part, decimal_part_trimmed);
}
