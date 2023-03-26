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

    for key in values_map.keys() {
        println!("{}", key);
    }

    println!("Integer to roman :{}", num);
    return "XXX".to_string();
}
