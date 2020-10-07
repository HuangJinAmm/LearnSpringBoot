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