//1431	Kids With the Greatest Number of Candies   

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> { 
    let mut v = Vec::new();
    let elements = candies.iter().max().unwrap();
    
    for candy in 0..candies.len() {    
        if candies[candy] + extra_candies >= *elements {
            v.push(true);
        } else {
            v.push(false);
        }
    }
    return v;
}