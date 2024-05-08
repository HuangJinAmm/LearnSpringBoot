use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut nums_set = HashSet::new();
    for num in nums {
        nums_set.insert(num);
    }
    let mut longest_streak = 0;
    for num in nums_set.iter() {
        let mut current_num = *num;
        if !nums_set.contains(&(current_num - 1)) {
            let mut current_streak = 1;
            while nums_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }
            longest_streak = std::cmp::max(longest_streak, current_streak);
        }
    }
    
    longest_streak
}

fn main() {
    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]),4);
    assert_eq!(longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]),9);
}