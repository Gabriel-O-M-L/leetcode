use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut converter: HashMap<char, i32> = HashMap::new();
        converter.insert('I', 1);
        converter.insert('V', 5);
        converter.insert('X', 10);
        converter.insert('L', 50);
        converter.insert('C', 100);
        converter.insert('D', 500);
        converter.insert('M', 1000);

        let mut result:i32 = 0;
        let s_chars: Vec<char> = s.chars().collect();
        for mut character in 0..s_chars.len() - 1 {
            if converter.get(&s_chars[character]).unwrap_or(&0) >= converter.get(&s_chars[character+1]).unwrap_or(&0){
                result += converter.get(&s_chars[character]).unwrap_or(&0);
            } else {
                result -= converter.get(&s_chars[character]).unwrap_or(&0);
            }
        }
        result += converter.get(&s_chars[s_chars.len() - 1]).unwrap_or(&0);
        result
    }
}