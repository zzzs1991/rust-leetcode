//给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
//
//
//
// 示例 1:
//
//
//输入: s = "abcabcbb"
//输出: 3
//解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
//
//
// 示例 2:
//
//
//输入: s = "bbbbb"
//输出: 1
//解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
//
//
// 示例 3:
//
//
//输入: s = "pwwkew"
//输出: 3
//解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
//
//
// 示例 4:
//
//
//输入: s = ""
//输出: 0
//
//
//
//
// 提示：
//
//
// 0 <= s.length <= 5 * 10⁴
// s 由英文字母、数字、符号和空格组成
//
// Related Topics 哈希表 字符串 滑动窗口 👍 6697 👎 0


//leetcode submit region begin(Prohibit modification and deletion)

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result = 0;
        let mut l = 0;
        let mut cache = vec![0; 128];
        s.chars().enumerate().for_each(|(i, ch)| {
            l = l.max(cache[ch as usize]);
            result = result.max(i as i32 - l + 1);
            cache[ch as usize] = i as i32  + 1;
        });
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)




#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        //输入: s = "abcabcbb"
        //输出: 3
        let a = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(a), 3);
        //输入: s = "bbbbb"
        //输出: 1
        let b = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(b), 1);

        //输入: s = "pwwkew"
        //输出: 3
        let c = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(c), 3);

        //输入: s = ""
        //输出: 0
        let d = "".to_string();
        assert_eq!(Solution::length_of_longest_substring(d), 0);

    }
}
