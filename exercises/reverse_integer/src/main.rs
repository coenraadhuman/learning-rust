pub fn reverse(x: i32) -> i32 {
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
            let mut value = reversed_value.unwrap();
            value = if x < 0 { value * -1 } else { value };
            value
        }
        _ => {
            0
        }
    }
}

fn main() {
    println!("For input: {} the result is {}.", 123, reverse(123));
    println!("For input: {} the result is {}.", -123, reverse(-123));
    println!("For input: {} the result is {}.", 120, reverse(120));
    println!("For input: {} the result is {}.", 0, reverse(0));
}