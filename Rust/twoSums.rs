impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vector: Vec<i32> = Vec::new();

        for x in 0..nums.len(){
            for y in x+1..nums.len(){
                if(nums[x]+nums[y] == target){
                    return vec![x as i32, y as i32];
                }
            }
        }
        vec![]
    }
}