/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}


impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
          Some(node) => match node.borrow_mut() {
            mut p => (p.left.is_none() && p.right.is_none() && p.val == target_sum)
                                      || Self::has_path_sum(p.left.take(), target_sum - p.val)
                                      || Self::has_path_sum(p.right.take(), target_sum - p.val)
          }
          _ => false,
        }
    }
}
// @lc code=end



#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_case_1_should_work() {
      let tree = prepare_tree();
      assert!(Solution::has_path_sum(tree, 22));
    }

    #[test]
    fn test_case_2_should_work() {
      let tree = prepare_tree();
      assert!(Solution::has_path_sum(tree, 17));
    }

    #[test]
    fn test_case_3_should_work() {
      let tree = prepare_tree();
      assert!(!Solution::has_path_sum(tree, 6));
    }


    fn prepare_tree() -> Option<Rc<RefCell<TreeNode>>> {
      /*
      [5,4,8,11,null,13,4,7,2,null,null,null,1]
      22
      */
      let to_assem = vec![5,4,8,11,-1,13,4,7,2,-1,-1,-1,1];
      helper(0 as usize, &to_assem)
    }

    fn helper(target: usize, source: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
      match source.get(target) {
          Some(val) => {
            let node = Rc::new(RefCell::new(TreeNode::new(val.to_owned())));
            node.borrow_mut().left = helper((target * 2 + 1) as usize, source);
            node.borrow_mut().right = helper((target * 2 + 2) as usize, source);
            Some(node)
          },
          None => None,
      }
    }
}
