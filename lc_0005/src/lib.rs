//给你一个字符串 s，找到 s 中最长的回文子串。
//
//
//
// 示例 1：
//
//
//输入：s = "babad"
//输出："bab"
//解释："aba" 同样是符合题意的答案。
//
//
// 示例 2：
//
//
//输入：s = "cbbd"
//输出："bb"
//
//
// 示例 3：
//
//
//输入：s = "a"
//输出："a"
//
//
// 示例 4：
//
//
//输入：s = "ac"
//输出："a"
//
//
//
//
// 提示：
//
//
// 1 <= s.length <= 1000
// s 仅由数字和英文字母（大写和/或小写）组成
//
// Related Topics 字符串 动态规划 👍 4545 👎 0


use std::cmp::max;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {return s;}

        let mut start = 0;
        let mut end = 0;
        let mut len = 1;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            let len1 = expandAroundCenter(&chars, i, i);
            let len2 = expandAroundCenter(&chars, i, i + 1);
            len = len1.max(len2);
            if len > end - start + 1 {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }
        s[start..=end].to_string()
    }
}

fn expandAroundCenter(array: &Vec<char>, l: usize, r: usize) -> usize {
    let mut left = l as i32;
    let mut right = r as i32;

    while left >= 0 && right < array.len() as i32 && array[left as usize] == array[right as usize] {
        left -= 1;
        right += 1;
    }
    (right - 1  - (left + 1) + 1) as usize
}


//leetcode submit region end(Prohibit modification and deletion)

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        //输入：s = "babad"
        //输出："bab"
        //解释："aba" 同样是符合题意的答案。
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
        //输入：s = "cbbd"
        //输出："bb"
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
        //输入：s = "a"
        //输出："a"
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
        //输入：s = "ac"
        //输出："a"
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
    }

    #[test]
    fn test_2() {
       let str =  "0123456".to_string();
        println!("{}", &str[1..2]);
        println!("{}", &str[2..3]);
        println!("{}", &str[3..4]);

    }
}
