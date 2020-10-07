#![allow(dead_code)]

#[cfg(test)]
mod tests{
    #[test]
    fn test_array() {
        let mut a:[i32;3] = [0;3];
        a[1] = 1;
        a[2] = 2;

        for x in &a{
            print!("{}",x)
        }
    }

    #[test]
    fn test_BTreeMap() {
        use std::collections::btree_map::BTreeMap;
        let mut count = BTreeMap::new();
        let message = "she sells sea shells by the sea shore";

        for c in message.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        assert_eq!(count.get(&'s'), Some(&8));

        println!("Number of occurrences of each character");
        for (char, count) in &count {
            println!("{}: {}", char, count);
        }
    }
}