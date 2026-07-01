/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx = nums1.len().saturating_sub(1);
        let (mut l, mut r) = (m - 1, n - 1);
        while r >= 0 {
            if l >= 0 && nums1[l as usize] > nums2[r as usize] {
                nums1[idx as usize] = nums1[l as usize];
                l -= 1;
            } else {
                nums1[idx as usize] = nums2[r as usize];
                r -= 1
            }
            idx -= 1;
        }
    }
}
// @lc code=end

