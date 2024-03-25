impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0{
            return false;
        }
        let digits: Vec<i32> = x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut inverse = digits.clone();
        inverse.reverse();
        let mut checker: bool = true;
        for x in 0..digits.len(){
            if(digits[x] != inverse[x]){
                checker = false;
            }
        }
        checker
    }
}