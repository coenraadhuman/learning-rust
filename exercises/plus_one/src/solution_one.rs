/*
This solution look at creating the integer value giving in the array, thereafter incrementing it and then changing it back to an array.

The issue with this solution is that parse the value will be too large for an u128 data type.
*/

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut new_digits = vec![];
    {
        let mut digits_as_string = vec![];
        let mut starts_with_x_zero = 0;
        let mut passed_start = false;
        let digits_len = digits.len();

        for digit in digits {
            if digit.eq(&0) && !passed_start && digits_len > 1 {
                starts_with_x_zero = starts_with_x_zero + 1;
            } else {
                passed_start = true;
            }
            digits_as_string.push(digit.to_string());
        }

        match digits_as_string.into_iter().collect::<String>().parse::<u128>() {
            Ok(y) => {
                for _ in 1..starts_with_x_zero {
                    new_digits.push(0);
                }
                for digit in (y + 1).to_string().chars() {
                    match digit.to_digit(10) {
                        None => {
                            println!("Digit conversion failed.");
                        }
                        Some(x) => {
                            new_digits.push(x as i32);
                        }
                    }
                }
            }
            Err(z) => {
                println!("Parse error: {}", z);
            }
        }
    }
    new_digits
}


