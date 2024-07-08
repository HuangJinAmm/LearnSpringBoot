use std::vec;

pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    let mut walk = 0;
    let mut water = capacity;

    for (pos, plant) in plants.iter().enumerate() {
        if water - plant < 0 {
            // 重新装水
            walk += (pos as i32) * 2;
            // 重置水量
            water = capacity;
        }
        water -= plant;
        walk += 1;
    }
    walk
}

pub fn main() {
    assert_eq!(watering_plants(vec![1, 1, 1, 4, 2, 3], 4), 30);
}
