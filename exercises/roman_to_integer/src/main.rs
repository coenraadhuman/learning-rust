pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;

    let get_integer_value = | roman_value: Option<char> | {
        if roman_value.is_none() {
            return 0;
        } else {
            return match roman_value.unwrap() {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0
            };
        }
    };

    let mut add_to_result = | x: i32 | {
        result = result + x;
    };

    let mut index: i32 = (s.chars().count() - 1) as i32;

    while index >= 0 {
        let current_value = get_integer_value(s.chars().nth(index as usize));
        let next_value = get_integer_value(s.chars().nth((index - 1) as usize));

        if next_value == 0 {
            add_to_result(current_value);
            index = -1;
        } else {
            if current_value == next_value{
                add_to_result(current_value);
                index = index - 1;
            } else if current_value < next_value {
                add_to_result(current_value);
                index = index - 1;
            } else {
                add_to_result(current_value - next_value);
                index = index - 2;
            }
        }
    }

    result
}

fn main() {
    println!("Input: {}, integer value = {}.", "III", roman_to_int(String::from("III")));
    println!("Input: {}, integer value = {}.", "IV", roman_to_int(String::from("IV")));
    println!("Input: {}, integer value = {}.", "IX", roman_to_int(String::from("IX")));
    println!("Input: {}, integer value = {}.", "LVIII", roman_to_int(String::from("LVIII")));
    println!("Input: {}, integer value = {}.", "MCMXCIV", roman_to_int(String::from("MCMXCIV")));
}
