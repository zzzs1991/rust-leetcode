//给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
//
// 请你将两个数相加，并以相同形式返回一个表示和的链表。
//
// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
//
//
//
// 示例 1：
//
//
//输入：l1 = [2,4,3], l2 = [5,6,4]
//输出：[7,0,8]
//解释：342 + 465 = 807.
//
//
// 示例 2：
//
//
//输入：l1 = [0], l2 = [0]
//输出：[0]
//
//
// 示例 3：
//
//
//输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//输出：[8,9,9,9,0,0,0,1]
//
//
//
//
// 提示：
//
//
// 每个链表中的节点数在范围 [1, 100] 内
// 0 <= Node.val <= 9
// 题目数据保证列表表示的数字不含前导零
//
// Related Topics 递归 链表 数学 👍 7308 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
//
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut current = &mut result;
        let mut group = (l1, l2, 0, 0);// (curr1, curr2, sum%10, sum/10)
        loop {
            group = match group {
                (None, None, _, 0) => break,
                (None, None, _, offset)
                    => (None, None, offset, 0),
                (Some(list), None, _, offset) | (None, Some(list), _, offset) => {
                    let s = list.val + offset;
                    (list.next, None, s % 10, s / 10)
                },
                (Some(list1), Some(list2), sum, offset) => {
                    let s = list1.val + list2.val + offset;
                    (list1.next, list2.next, s % 10, s / 10)
                },
            };
            *current = Some(Box::new(ListNode::new(group.2)));
            current =  &mut current.as_mut().unwrap().next;
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    //输入：l1 = [2,4,3], l2 = [5,6,4]
    //输出：[7,0,8]
    //解释：342 + 465 = 807.
    #[test]
    fn test_1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None
                }))
            }))
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));
        let result = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: None
                }))
            }))
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }
}
