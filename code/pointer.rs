/**********************************************
  > File Name		: pointer.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Thu 18 Feb 2021 10:26:24 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::ptr::NonNull;

struct TreeNode<T: Ord> {
    val: T,
    height: u32,
    left: Option<NonNull<TreeNode<T>>>,
    right: Option<NonNull<TreeNode<T>>>
}

impl<T> TreeNode<T> where T: Ord {
    pub fn new(val: T, height: u32) -> Self {
        TreeNode {
            val,
            height,
            left: None,
            right: None,
        }
    }
}

struct Tree<T: Ord> {
    root: Option<NonNull<TreeNode<T>>>
}

fn main() {
    let mut n4 = Some(unsafe {NonNull::new_unchecked(Box::new(TreeNode::new(4,1)))});
    let mut n6 = Some(unsafe {NonNull::new_unchecked(Box::new(TreeNode::new(4,1)))});
    let mut n8 = Some(unsafe {NonNull::new_unchecked(Box::new(TreeNode::new(4,1)))});
    match &n4 {
        None => {},
        Some(v) => unsafe {
            (*v.as_ptr()).right = n6.clone();
        }
    }
    match &n6 {
        None => {},
        Some(v) => unsafe {
            (*v.as_ptr()).right = n8.clone();
        }
    }
    let temp = match &n4 {
        None => None,
        Some(v) => unsafe {
            (*v.as_ptr()).right.clone()
        }
    };
    let right_left = match &temp {
        None => None,
        Some(v) => unsafe {
            (*v.as_ptr()).left.clone()
        }
    };
    if let Some(v) = &n4 {
        (*v.as_ptr()).right = right_left.clone();
    }
    if let Some(v) = &temp {
        (*v.as_ptr()).left = n4.clone();
    }
}
