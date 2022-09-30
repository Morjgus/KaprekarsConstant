// Kaprekar's constant

use itertools::Itertools;


fn main() {
    let mut val = 123;

    loop {
        let val_str = val.to_string();
        let large_chars = val_str.chars().sorted().rev();
        let large_str: String = large_chars.collect::<String>();
        let large_val: i32 = large_str.parse().unwrap();
        let small_chars = val_str.chars().sorted();
        let small_str: String = small_chars.collect::<String>();
        let small_val: i32 = small_str.parse().unwrap();
        let next_val = large_val - small_val;
        println!("val: {}, large: {}, small: {}, next: {}", val, large_val, small_val, next_val);
        if next_val == val {
            break;
        }
        val = next_val;

    }

    println!("Hello, world!");
}
