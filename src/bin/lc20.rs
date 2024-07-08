/// 20. 有效的括号
fn main() {
    print!("{}", is_valid("()[]{}"));
}

fn is_valid(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            }
            ')' => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if top != '(' {
                    return false;
                }
            }
            ']' => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if top != '[' {
                    return false;
                }
            }
            '}' => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if top != '{' {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }
    return stack.is_empty();
}
