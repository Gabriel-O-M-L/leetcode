use std::collections::HashMap;
impl Solution {
        pub fn is_valid(s: String) -> bool {
            let mut stack:Vec<char> = Vec::new();
            let mut brackets: HashMap<char, char> = HashMap::new();
            brackets.insert(')', '(');
            brackets.insert('}', '{');
            brackets.insert(']', '[');

            for chara in s.chars(){
                if chara == '{' || chara == '[' || chara == '(' {
                    stack.push(chara);
                } else if brackets.contains_key(&chara) {
                    if stack.is_empty() || *brackets.get(&chara).unwrap() != stack.pop().unwrap() {
                        return false;
                    }
                }
            }
            stack.is_empty()
        }
}