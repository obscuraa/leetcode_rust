// 1365 How Many Numbers Are Smaller Than the Current Number
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {  
    let mut vec = Vec::new();
    
    for index in 0..nums.len() {        
        let mut counter: i32 = 0;         
        for cmp_index in 0..nums.len() {
           if index != cmp_index && nums[index] > nums[cmp_index]
            {
               counter += 1;
            }
        }
        vec.push(counter);
    }
    
    return vec;
}