#![allow(dead_code)]

fn good_enuough(guess: f64, x: f64) -> bool {
    let diff = guess * guess - x;
    if diff.abs() < 0.00001f64 {
        true
    } else {
        false
    }
}

fn average(x: f64, y: f64) -> f64 {
    (x + y) / 2f64
}

fn improve(guess: f64, x: f64) -> f64 {
    average(guess, x / guess)
}

fn sqrt_iter(guess: f64, x: f64) -> f64 {
    if good_enuough(guess, x) {
        guess
    } else {
        sqrt_iter(improve(guess, x), x)
    }
}

fn n_sqrt(x: f64) -> f64 {
    sqrt_iter(1f64, x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sqrt() {
        let x = n_sqrt(2f64);
        println!("{}", x);
    }
}
