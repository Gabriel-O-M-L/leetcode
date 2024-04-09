class Solution {
public:
    std::vector<int> twoSum(std::vector<int>& nums, int target) {
        if (nums.size() == 2) {
            return {0, 1};
        }
        unordered_map<int,int> map;
        for(int i=0;i<nums.size();i++){
            auto second_value = target - nums[i];
            if(map.find(second_value)!= map.end() && map[second_value] != i){
                return{i, map[second_value]};
            }else{
                map[nums[i]] = i;
            }
        }
        return {};
    }
};