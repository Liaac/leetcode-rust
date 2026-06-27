/*
 * @lc app=leetcode.cn id=633 lang=rust
 *
 * [633] 平方数之和
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut a = 0;
        let mut b = (c as f64).sqrt() as i64;
        while a <= b {
            let sum = a * a + b * b;
            if sum == c {
                return true;
            } else if sum < c {
                a += 1;
            } else {
                b -= 1;
            }
        }
        false
    }
}
// @lc code=end

