
use std::collections::HashMap;
use crate::solution::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            let index = i as i32;
            m.entry(v).or_insert(index);
            if let Some(x) = m.get(&(target - v)) {
                if *x != index {
                    return vec![*x, index];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::two_sum(vec![3, 3], 6), vec![0, 1]
        );
        assert_eq!(
            Solution::two_sum(vec![1, 2, 5], 7), vec![1, 2]
        );
    }
}
