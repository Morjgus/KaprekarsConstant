// Kaprekar's constant

use itertools::Itertools;
use std::io;


fn main() {
    let mut val = String::new();
    println!("Please inut a 3 or 4 digits number:");
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");
    let mut val: u32 = val.trim().parse()
        .expect("Pleae type a number");

    loop {
        let val_str = val.to_string();
        let large_chars = val_str.chars().sorted().rev();
        let large_str: String = large_chars.collect::<String>();
        let large_val: u32 = large_str.parse().unwrap();
        let small_chars = val_str.chars().sorted();
        let small_str: String = small_chars.collect::<String>();
        let small_val: u32 = small_str.parse().unwrap();
        let next_val = large_val - small_val;
        println!("val: {}, large: {}, small: {}, next: {}", val, large_val, small_val, next_val);
        if next_val == val {
            break;
        }
        val = next_val;

    }
    
    if val == 495 || val == 6174 {
        println!("Kaprekar's constant is: {}", val);
    } else {
        println!("Couldn't compute Kaprekar's constant. Invalid inpout.");
    }
}
