#![allow(dead_code)]

fn factorial(n:u32) -> u32{
    if n==1 {
        1
    } else {
        n*factorial(n-1)
    }
}

fn fact_iter(product:u32, counter:u32,max_count:u32) -> u32 {
    if counter > max_count {
        product
    } else {
        fact_iter(product*counter, counter+1, max_count)
    }
}

fn fixed_point(f:impl Fn(f64) -> f64,first_guess:f64) -> f64 {
    let next = f(first_guess);
    println!("{}",next);
    let t = first_guess - next;
    if t.abs() < 0.00000001f64 {
        next
    } else {
        fixed_point(f, next)
    }
}

const DX:f64 = 0.0000001;

fn deriv(f:impl Fn(f64)->f64,x:f64) -> f64 {
    (f(x+DX) - f(x))/DX
}

fn cube(x:f64) -> f64 {
    x.powi(3)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fixed_pont() {
        // let f = |x:f64| x.cos();
        let g = 2.0f64;
        let f = move |x:f64| ->f64 {
            1.0+1.0/x
        };
        let retn = fixed_point(f, g);
        println!("{}",retn);
    }

    #[test]
    fn test_deriv() {
        let retn = deriv(cube, 5.0f64);
        println!("{}",retn);
    }
}