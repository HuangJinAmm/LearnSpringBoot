use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            let pos = i as i32;
            if let Some(k) = map.get(&(target - nums[i])) {
                if *k != pos {
                    return vec![*k, pos];
                }
            }
            map.insert(nums[i], pos);
        }
        panic!("not found!")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_two_sum() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        println!("{:?}", result);
    }
}
