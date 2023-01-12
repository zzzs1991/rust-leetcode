use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

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

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let l = Self::max_depth(&node.as_ref().borrow().left);
                let r = Self::max_depth(&node.as_ref().borrow().right);
                max(l, r) + 1
            }
            None => 0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        let mut to_test = TreeNode::new(1);
        to_test.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        to_test.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert_eq!(Solution::max_depth(&Some(Rc::new(RefCell::new(to_test)))), 2);
    }
}
