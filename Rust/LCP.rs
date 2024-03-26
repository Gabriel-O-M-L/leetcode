impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix:String = String::new();
        let mut result:bool = true;
        prefix = strs[0].clone();

        for i in 0..prefix.len() + 1{
            result = true;
            for x in strs.clone(){
                if !x.starts_with(&prefix[0..i]){
                    result = false;
                    break;
                }
            }
            if !result {
                if i == 0 {
                    return "".to_string();
                } else {
                    return prefix[0..i-1].to_string();
                }
            }
        }
        prefix
    }
}