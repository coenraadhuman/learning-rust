/*
This solution looks at recursion to increment the last value in the array and checking whether it overflows to the position before.
*/

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut my_digits = digits;
    let mut index = my_digits.len() - 1;

    loop {
        match my_digits.get(index) {
            None => {
                println!("Not found.")
            }
            Some(_) => {
                if my_digits[index].eq(&9) {
                    my_digits[index] = 0;
                    if index.eq(&0) {
                        my_digits.insert(0, 0);
                    } else {
                        index = index - 1;
                    }
                } else {
                    my_digits[index] = my_digits[index] + 1;
                    break;
                }
            }
        }
    }

    my_digits
}


