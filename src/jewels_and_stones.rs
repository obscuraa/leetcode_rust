//771 Jewels and Stones

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    return s.chars().filter(|&stone| j.contains(stone)).count() as i32;
}