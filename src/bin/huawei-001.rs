use std::io::{self, *};

fn main() {
    let stdin = io::stdin();
    unsafe {
        for line in stdin.lock().lines() {
            let ll = line.unwrap();
            let input_num = ll.trim().parse::<i32>().unwrap_or(0);
            if input_num == 0 {
                break;
            }
            print!("{}\n", input_num / 2);
        }
    }
}
