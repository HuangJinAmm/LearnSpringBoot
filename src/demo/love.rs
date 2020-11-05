fn love(s:String) {
    let mut sl : Vec<String> = Vec::new();
    let v : Vec<char> = s.chars().collect(); 
    let xf : f32 = 0.05;
    let yf : f32 = 0.1;
    for y in -15..=15 {
        let mut stemp = String::new();
        for x in -30..=30 {
            if (((x as f32)*xf ).powi(2)+((y as f32)*yf).powi(2)-1f32).powi(3)-((x as f32)*xf).powi(2)*((y as f32)*yf).powi(3) <= 0f32 {
                let l = s.len() as i32;
                let mut p = (x - y)%(l);
                if p < 0{
                    p = l + p ;
                }
                if let Some(x) = v.get(p as usize){
                    stemp.push(x.clone());
                }
            } else {
                stemp.push(' ');
            }
        }
        sl.push(stemp);
    }
    for l in sl {
        println!("{}",l)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_love() {
        love(String::from("Love"));
    }
}