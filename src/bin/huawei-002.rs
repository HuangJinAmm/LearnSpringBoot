use std::io::{self, *};

fn main() {
    let stdin = io::stdin();
    unsafe {
        for line in stdin.lock().lines() {
            let ll = line.unwrap().to_ascii_uppercase();
            let hex_str = &ll[2..];
            let mut hex_arr: Vec<_> = hex_str.chars().collect();
            hex_arr.reverse();
            println!("{:#?}", hex_arr);
            let mut hex = 0u32;
            for (i, ele) in hex_arr.into_iter().enumerate() {
                if let Some(digit) = ele.to_digit(16) {
                    let d = digit * (16u32.pow(i as u32)) as u32;
                    hex += d;
                } else {
                    println!("非法字符");
                    return;
                }
            }
            println!("{}", hex);
        }
    }
}
