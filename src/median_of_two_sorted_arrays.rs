use crate::solution::Solution;
use std::cmp;

fn get_lr_by_offset(nums: &Vec<i32>, offset: i32) -> (i32, i32) {
    let len = nums.len() as i32;
    if len == 0 {
        return (std::i32::MIN, std::i32::MAX)
    }
    let (lidx, ridx) = ((len + offset + 1) / 2 - 1, (len + offset + 2) / 2 - 1);
    if lidx < 0 {
        (std::i32::MIN, nums[ridx as usize])
    } else if ridx == len {
        (nums[lidx as usize], std::i32::MAX)
    } else {
        (nums[lidx as usize], nums[ridx as usize])
    }
}
 
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums_a, nums_b) = if nums1.len() >= nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let abs_offset = nums_a.len() as i32;
        let (mut left, mut right) = (-abs_offset, abs_offset);
        let mut mid = (left + right + 2 * abs_offset) / 2 - abs_offset;
        loop {
            let (la, ra) = get_lr_by_offset(&nums_a, mid);
            let (lb, rb) = get_lr_by_offset(&nums_b, -mid);
            if la > rb {
                right = mid;
            } else if ra < lb {
                if left + 1 == right {
                    left = right;
                } else {
                    left = mid;
                }
            } else {
                return (cmp::max(la, lb) + cmp::min(ra, rb)) as f64 / 2.0;
            }
            mid = (left + right + 2 * abs_offset) / 2 - abs_offset;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    #[test]
    fn test_solution() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![3], vec![-2, -1]), -1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![100001], vec![100000]), 100000.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![1, 3, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![4], vec![1, 2, 3]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4, 5]), 3.0);
    }
}