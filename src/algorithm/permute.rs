#![allow(dead_code)]

pub fn permute(nums: Vec<i64>) ->Vec<Vec<i64>>{
    let mut track:Vec<i64> = Vec::new();
    let mut result:Vec<Vec<i64>> = Vec::new();
    back_track(&mut result, &nums, &mut track);
    result
}

fn back_track(result:& mut Vec<Vec<i64>>,nums:&Vec<i64>,track:&mut Vec<i64>) {

    if nums.len() == track.len(){
        result.push(track.to_owned());
        return ; 
    }

    for i in 0..nums.len() {
        if track.contains(&nums[i]){
            continue;
        }
        track.push(nums[i]);
        back_track(result, nums, track);
        track.pop();
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_permute() {
        let nums = vec![1,3,5,7];
        let result = permute(nums);
        println!("{:?}",result);
    }
}