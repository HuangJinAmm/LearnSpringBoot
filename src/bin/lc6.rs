///6. Z 字形变换
fn main() {
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 3;
    let result = convert(s, num_rows);
    println!("{}", result);
}

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() <= num_rows as usize {
        return s;
    }
    let mut rows = vec![String::new(); num_rows as usize];
    let mut cur_row = 0;
    let mut going_down = false;
    for c in s.chars() {
        rows[cur_row as usize].push(c);
        if cur_row == 0 || cur_row == num_rows - 1 {
             going_down = !going_down;
        }
        cur_row += if going_down { 1 } else { -1 };
    }
    rows.join("")
}