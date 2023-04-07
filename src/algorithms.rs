use std::cmp::max;

pub fn maximum_count(nums: Vec<i32>) -> i32 {
    max(
        nums.iter().filter(|&&x| x < 0).count(),
        nums.iter().filter(|&&x| x > 0).count(),
    ) as i32
}

pub fn maximum_count_for(nums: Vec<i32>) -> i32 {
    let mut pos: i32 = 0;
    let mut neg: i32 = 0;
    for i in nums {
        if i.is_positive() {
            pos += 1;
        } else if i.is_negative() {
            neg += 1;
        }
    }
    return max(pos, neg);
}
