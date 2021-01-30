pub fn is_palindrome(x: i32) -> bool {
    if x.to_string().chars().count() == 1 {
        return true;
    } else if x < 0 {
        return false;
    } else {
        let mut reversed_string_value = vec![];

        for (index, character) in x.to_string().chars().rev().enumerate() {
            if index == 0 && character != '0' && character.is_digit(10) {
                reversed_string_value.push(character);
            } else if index > 0 && character.is_digit(10) {
                reversed_string_value.push(character);
            }
        }

        let reversed_value = reversed_string_value.into_iter().collect::<String>().parse::<i32>();
        match reversed_value {
            Ok(_) => {
                let value = reversed_value.unwrap();
                if x == value {
                    return true;
                }
                return false;
            }
            _ => {
                return false;
            }
        }
    }
}

fn main() {
    println!("Input {}, is it a palindrome: {}", 121, is_palindrome(121));
    println!("Input {}, is it a palindrome: {}", 2, is_palindrome(2));
    println!("Input {}, is it a palindrome: {}", -121, is_palindrome(-121));
    println!("Input {}, is it a palindrome: {}", 23, is_palindrome(23));
    println!("Input {}, is it a palindrome: {}", 54645, is_palindrome(54645));
    println!("Input {}, is it a palindrome: {}", 101, is_palindrome(101));
    println!("Input {}, is it a palindrome: {}", 22, is_palindrome(22));
    println!("Input {}, is it a palindrome: {}", 11, is_palindrome(11));
    println!("Input {}, is it a palindrome: {}", 13, is_palindrome(13));
}
