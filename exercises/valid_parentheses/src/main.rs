pub fn is_valid(s: String) -> bool {
    if s.chars().count() % 2 != 0 {
        return false;
    }

    let mut bracket_stack:Vec<char> = vec![];

    for character in s.chars() {
        match character {
            '{' => bracket_stack.push('{'),
            '[' => bracket_stack.push('['),
            '(' => bracket_stack.push('('),
            '}' => {
                match bracket_stack.get(bracket_stack.len() - 1) {
                    None => {
                        return false;
                    }
                    Some('{') => {
                        bracket_stack.pop();
                    },
                    Some(_) => {
                        return false;
                    }

                }
            },
            ']' => {
                match bracket_stack.get(bracket_stack.len() - 1) {
                    None => {
                        return false;
                    }
                    Some('[') => {
                        bracket_stack.pop();
                    },
                    Some(_) => {
                        return false;
                    }

                }
            },
            ')' => {
                match bracket_stack.get(bracket_stack.len() - 1) {
                    None => {
                        return false;
                    }
                    Some('(') => {
                        bracket_stack.pop();
                    },
                    Some(_) => {
                        return false;
                    }

                }
            },
            _ => return false
        }
    }

    if bracket_stack.len() > 0 {
        return false;
    }

    true
}

fn main() {
    println!("Input: {}, result: {}.", String::from("()"), is_valid(String::from("()")));
    println!("Input: {}, result: {}.", String::from("()[]{}"), is_valid(String::from("()[]{}")));
    println!("Input: {}, result: {}.", String::from("(]"), is_valid(String::from("(]")));
    println!("Input: {}, result: {}.", String::from("([)]"), is_valid(String::from("([)]")));
    println!("Input: {}, result: {}.", String::from("{[]}"), is_valid(String::from("{[]}")));
}
