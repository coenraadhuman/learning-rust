fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut index_a = 0;
    let mut index_b = 0;
    let mut found = false;

    for element_a in nums.iter() {
        if !found {
            for element_b in nums.iter() {
                if index_a != index_b {
                    if element_a + element_b == target {
                        result.push(index_a);
                        result.push(index_b);
                        found = true;
                    }
                }
                index_b = index_b + 1;
            }
            index_a = index_a + 1;
            index_b = 0;
        }
    }
    result
}

fn main() {
    print!("Result is {:?}", two_sum(vec![2,7,11,15], 9));
}

