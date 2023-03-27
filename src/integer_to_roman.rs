// Convert integer to roman numerals
// https://leetcode.com/problems/integer-to-roman/

use std::collections::HashMap;

pub fn run(num: i32) -> String {
    let values_map = HashMap::from(
        [
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000)
        ]
    );

    println!("Converting: {}", num.to_string());
    for key in values_map.keys() {
        let value = values_map.get(key);
        println!("{} | {}", key, value.unwrap());
    }

    

    return "XXX".to_string();
}
