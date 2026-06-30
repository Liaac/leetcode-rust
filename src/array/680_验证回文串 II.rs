/*
 * @lc app=leetcode.cn id=680 lang=rust
 *
 * [680] 验证回文串 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let cur= s.as_bytes();
        let (mut left, mut right) = (0, cur.len().saturating_sub(1));
        while left < right {
            if cur[left] != cur[right] {
                return is_palindrome(cur, left+1, right) || is_palindrome(cur, left, right-1);
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

fn is_palindrome(s: &[u8], mut l: usize, mut r: usize) -> bool {
    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
// @lc code=end

