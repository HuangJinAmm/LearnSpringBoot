use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut ans = 0i32;
        let mut tmp = 0i32;
        for (i, c) in s.chars().enumerate() {
            let lastpos = *map.get(&c).unwrap_or(&-1);
            let i = i as i32;
            map.insert(c, i);
            if tmp < i - lastpos {
                tmp += 1;
            } else {
                tmp = i - lastpos;
            }
            ans = ans.max(tmp);
        }
        ans
    }
}
