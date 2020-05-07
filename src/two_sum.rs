use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for index in 0..nums.len() {
        for cmp_index in index + 1..nums.len() { 
            if nums[index] + nums[cmp_index] == target
            {
                return vec![index as i32, cmp_index as i32];
            }
        }
    }
    return vec![0;1]; 
}

pub fn two_sum_faster(nums: Vec<i32>, target: i32) -> Vec<i32> {    
    let mut map = HashMap::new();
        
    for index in 0..nums.len() {
        let complement = target - nums[index] as i32;
            
        if let Some(val) = map.get(&complement) {
            return vec![*val, index as i32];
        } else {
            map.insert(target - &complement, index as i32);
        }
        
        map.insert(nums[index], index as i32);
    } 
    return vec![0;1];
}
