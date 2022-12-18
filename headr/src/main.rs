use std::collections::HashMap;

fn main() {
    // if let Err(e) = headr::get_args().and_then(headr::run) {
    //     eprintln!("{}", e);
    //     std::process::exit(1);
    // }

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // println!("{:#?}", scores);

    println!("{:#?}", two_sum(vec![1, 2, 3], 4));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut cmp = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = cmp.get(&(target - num)) {
            result.push(i as i32);
            result.push(j as i32);
            return result;
        }
        cmp.insert(num, i as i32);
    }
    result
}
