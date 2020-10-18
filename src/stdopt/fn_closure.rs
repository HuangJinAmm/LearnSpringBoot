#![allow(dead_code)]
fn call_fn(x:String,func: impl Fn(String)){
    func(x);
}

#[cfg(test)]
mod tests{

    #[test]
    fn test_call_fn() {
        let g = "ggggg".to_string();
        let f = move |x:String| println!("{} and {}",x,g);
        super::call_fn("xxxx".to_string(), f);
    }
}