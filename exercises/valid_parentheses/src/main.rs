pub fn is_valid(s: String) -> bool {
    if s.chars().count() % 2 != 0 {
        return false;
    }

    

    true
}

fn main() {
    println!("Input: {}, result: {}.", String::from(""), is_valid(String::from("")));
    println!("Input: {}, result: {}.", String::from(""), is_valid(String::from("")));
    println!("Input: {}, result: {}.", String::from(""), is_valid(String::from("")));
    println!("Input: {}, result: {}.", String::from(""), is_valid(String::from("")));
    println!("Input: {}, result: {}.", String::from(""), is_valid(String::from("")));
    println!("Input: {}, result: {}.", String::from(""), is_valid(String::from("")));
}
