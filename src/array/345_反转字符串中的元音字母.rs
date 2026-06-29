/*
 * @lc app=leetcode.cn id=345 lang=rust
 *
 * [345] 反转字符串中的元音字母
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut newString = Vec::new();
        let mut indexes = Vec::new();
        for (i, ch) in s.chars().enumerate() {
            newString.push(ch);
            if isyuanyin(ch) {
                indexes.push(i);
            }
        }
        if indexes.is_empty() {
            return s;
        }
        let (mut left, mut right) = (0, indexes.len()-1);
        while left < right {
            newString.swap(indexes[left], indexes[right]);
            left += 1;
            right -= 1;
        }
        newString.into_iter().collect()
    }
}

fn isyuanyin(ch: char) -> bool {
    ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'
        || ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U'
}

// pub fn reverse_vowels(s: String) -> String {
//     let mut chars: Vec<char> = s.chars().collect();
//     let (mut l, mut r) = (0, chars.len().saturating_sub(1));
//     while l < r {
//         if !is_vowel(chars[l]) {
//             l += 1;
//         } else if !is_vowel(chars[r]) {
//             r -= 1;
//         } else {
//             chars.swap(l, r);
//             l += 1;
//             r -= 1;
//         }
//     }
//     chars.into_iter().collect()
// }

// fn is_vowel(ch: char) -> bool {
//     matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O'
// | 'U')
// }
// @lc code=end

