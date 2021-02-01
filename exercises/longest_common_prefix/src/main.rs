pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut common_prefix: Vec<char> = vec![];

    if strs.len() <= 0 {
        return String::from("");
    } else {
        let mut common_char = true;
        let mut char_index = 0;

        while common_char {
            let mut option_current_char: Option<char> = Option::None;

            for (index, string) in strs.iter().enumerate() {
                match string.chars().nth(char_index) {
                    Some(string_char) => {
                        if common_char {
                            if index == 0 {
                                option_current_char = Option::Some(string_char);
                            } else {
                                match option_current_char {
                                    Some(current_char) => {
                                        if string_char != current_char {
                                            common_char = false;
                                        }
                                    },
                                    None => common_char = false
                                }
                            }
                        }
                    },
                    None => {
                        common_char = false;
                    }
                }
            }
            if common_char {
                match option_current_char {
                    Some(x) => common_prefix.push(x),
                    None => common_char = false
                }
            }
            char_index = char_index + 1;
        }
    }

    common_prefix.into_iter().collect()
}

fn main() {
    println!("Input: {:?}, output: {}", vec![String::from("flower"), String::from("flow"), String::from("flight")], longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]));
    println!("Input: {:?}, output: {}", vec![String::from("dog"), String::from("racecar"), String::from("car")], longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]));
}
