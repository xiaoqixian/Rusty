/**********************************************
  > File Name		: AVLTree.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue 16 Feb 2021 05:02:38 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

/*
 * Implement a average length tree in Rust.
 */

use std::cmp::Ordering;
use std::ptr::NonNull;
use std::mem::drop;
use std::fmt::Display;
/*
 * Using NonNull pointer can save some space,
 * cause we use None to represent null pointer 
 * and None takes no memory. 
 */

#[allow(unused)]
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

impl<T> Tree<T> where T: Ord + Clone + Display {
    pub fn new() -> Self {
        Tree {
            root: None
        }
    }

    fn rotate_left(&self, node: Option<NonNull<TreeNode<T>>>) -> Option<NonNull<TreeNode<T>>> {
        match node {
            None => None,
            Some(n) => {
                unsafe {
                    let temp = (*n.as_ptr()).right.clone();
                    match &temp {
                        None => {
                            println!("Can't do rotate_left operation on a node without right subtree.");
                            assert!(false);
                        },
                        Some(rn) => {
                            (*n.as_ptr()).right = (*rn.as_ptr()).left.clone();
                            (*rn.as_ptr()).left = node.clone();
                        }
                    }
                    self.update_node_height(node);
                    self.update_node_height(temp);
                    temp
                }
            }
        }
    }

    fn rotate_right(&self, node: Option<NonNull<TreeNode<T>>>) -> Option<NonNull<TreeNode<T>>> {
        match node {
            None => None,
            Some(n) => {
                unsafe {
                    let temp = (*n.as_ptr()).left.clone();
                    match &temp {
                        None => {
                            println!("Can't do rotate_right operation on a node without left subtree.");
                            assert!(false);
                        },
                        Some(ln) => {
                            (*n.as_ptr()).left = (*ln.as_ptr()).right.clone();
                            (*ln.as_ptr()).right = node.clone();
                        }
                    }
                    self.update_node_height(node);
                    self.update_node_height(temp);
                    temp
                }
            }
        }
    }

    fn update_node_height(&self, node: Option<NonNull<TreeNode<T>>>) {
        let mut max = 0;
        match node {
            None => {},
            Some(n) => unsafe {
                let left = &((*n.as_ptr()).left);
                let right = &((*n.as_ptr()).right);
                match left {
                    None => {},
                    Some(ln) => {
                        if (*ln.as_ptr()).height > max {
                            max = (*ln.as_ptr()).height;
                        }
                    }
                }
                match right {
                    None => {},
                    Some(rn) => {
                        if (*rn.as_ptr()).height > max {
                            max = (*rn.as_ptr()).height;
                        }
                    }
                }
                (*n.as_ptr()).height = max+1;
            }
        };
    }

    fn get_node_height(&self, node: &Option<NonNull<TreeNode<T>>>) -> u32 {
        match node {
            None => 0,
            Some(n) => {
                unsafe {
                    (*n.as_ptr()).height
                }
            }
        }
    }

    fn insert_with_node(&self, mut node: Option<NonNull<TreeNode<T>>>, val: T) -> Option<NonNull<TreeNode<T>>> {
        match &node {
            None => {
                let new_node = Box::new(TreeNode::new(val, 1));
                Some(unsafe {NonNull::new_unchecked(Box::into_raw(new_node))})
            },
            Some(n) => {
                let node_val: &T;
                let left: &mut Option<NonNull<TreeNode<T>>>;
                let right: &mut Option<NonNull<TreeNode<T>>>;
                unsafe {
                    node_val = &(*n.as_ptr()).val;
                    left = &mut(*n.as_ptr()).left;
                    right = &mut(*n.as_ptr()).right;
                }
                println!("The node val = {}", node_val);
                match node_val.cmp(&val) {
                    Ordering::Equal => {
                        println!("The AVLTree doesn't support duplicated node yet.");
                        return None;
                    },
                    Ordering::Greater => {
                        unsafe {
                            (*n.as_ptr()).left = self.insert_with_node((*n.as_ptr()).left, val.clone());
                            println!("insert left {}", &val);
                        }
                        if self.get_node_height(left) > self.get_node_height(right) + 1 {
                            println!("Need to rotate right.");
                            let mut left_val = val.clone();
                            match left {
                                None => {
                                    println!("This is not possible");
                                    assert!(false);
                                },
                                Some(ln) => {
                                    unsafe {
                                        left_val = (*ln.as_ptr()).val.clone();
                                    }
                                }
                            }
                            /*
                             * if the node is inserted on the right side of the left child.
                             */
                            if let Ordering::Greater = left_val.cmp(&val) {
                                println!("Need to rotate left first.");
                                unsafe {
                                    (*n.as_ptr()).left = self.rotate_left(*left);
                                }
                            }
                            println!("Then rotate right.");
                            node = self.rotate_right(node);
                        }
                    },
                    Ordering::Less => {
                        unsafe {
                            (*n.as_ptr()).right = self.insert_with_node((*n.as_ptr()).right, val.clone());
                            println!("insert right {}", &val);
                        }
                        if self.get_node_height(right) > self.get_node_height(left) + 1 {
                            println!("Need to roate left.");
                            let mut right_val = val.clone();
                            match right {
                                None => {
                                    println!("This is not possible.");
                                    assert!(false);
                                },
                                Some(rn) => {
                                    right_val = unsafe {(*rn.as_ptr()).val.clone()};
                                }
                            };
                            if let Ordering::Greater = right_val.cmp(&val) {
                                println!("Need to rotate right first.");
                                unsafe {
                                    (*n.as_ptr()).left = self.rotate_right(*right);
                                }
                            };
                            println!("Then rotate left.");
                            node = self.rotate_left(node);
                        }
                    }
                };
                self.update_node_height(node);
                self.print_node(&node);
                node
            }
        }
    }

    fn print_node(&self, node: &Option<NonNull<TreeNode<T>>>) {
        match node {
            None => {
                println!("None node");
            },
            Some(v) => unsafe {
                println!("Node {}'s height = {}", (*v.as_ptr()).val, (*v.as_ptr()).height);
            }
        }
    }

    fn remove_with_node(&self, mut node: Option<NonNull<TreeNode<T>>>, val: &T) -> Option<NonNull<TreeNode<T>>> {
        match &node {
            None => {
                return None;
            },
            Some(n) => {
                let node_val = unsafe {
                    &(*n.as_ptr()).val
                };
                let left = unsafe {
                    &((*n.as_ptr()).left)
                };
                let right = unsafe {
                    &((*n.as_ptr()).right)
                };
                match node_val.cmp(val) {
                    Ordering::Greater => unsafe {
                        (*n.as_ptr()).left = self.remove_with_node((*n.as_ptr()).left, val);
                        if self.get_node_height(right) > self.get_node_height(left) + 1 {
                            match right {
                                None => {
                                    println!("This is not possible.");
                                    assert!(false);
                                },
                                Some(_) => {
                                    if self.get_node_height(left) > self.get_node_height(right) + 1 {
                                        (*n.as_ptr()).right = self.rotate_right(*right);
                                    }
                                    node = self.rotate_right(node);
                                }
                            }
                        }
                    },
                    Ordering::Less => unsafe {
                        (*n.as_ptr()).right = self.remove_with_node((*n.as_ptr()).right, val);
                        if self.get_node_height(left) > self.get_node_height(right) + 1 {
                            match left {
                                None => {
                                    println!("The left node is None.");
                                    assert!(false);
                                },
                                Some(_) => {
                                    if self.get_node_height(right) - self.get_node_height(left) > 1 {
                                        (*n.as_ptr()).left = self.rotate_left(*left);
                                    }
                                    node = self.rotate_right(node);
                                }
                            }
                        }
                    },
                    //The current node is the node to be deleted
                    Ordering::Equal => unsafe {
                        //If the current node is a leaf, just delete it.
                        //Leave the balance problem to father nodes.
                        if let None = left {
                            if let None = right {
                                drop(n);
                                return None;
                            }
                        }
                        /*
                         * If the current node has only right subtree,
                         * then its right subtree must be leaf node, otherwise
                         * the height difference between the right subtree and
                         * the left subtree will be more than 1.
                         * So we can just rotate left and release the left node.
                         * Then the right subtree will still be a leaf node.
                         */
                        if let None = left {
                            let right_tree = (*n.as_ptr()).right;
                            drop(n);
                            return right_tree;
                        }
                        //So as the left subtree.
                        if let None = right {
                            let left_tree = (*n.as_ptr()).left;
                            drop(n);
                            return left_tree;
                        }
                        /*
                         * If both subtrees of the current node exist.
                         * Then copy the value from one of the child nodes, 
                         * and delete one of the child nodes by traversely 
                         * calling remove_with_node function. 
                         * We delete from the higher subtree, cause the deletion
                         * will decrease the height by 1 as most. So we don't 
                         * have to rebalance the tree.
                         * If two node are of the same height, delete from the 
                         * right subtree by default.
                         */
                        if self.get_node_height(right) >= self.get_node_height(left) {
                            match right {
                                None => {
                                    println!("The right node is None.");
                                    assert!(false);
                                },
                                Some(rn) => {
                                    (*n.as_ptr()).val = (*rn.as_ptr()).val.clone();
                                    (*n.as_ptr()).right = self.remove_with_node(*right, &(*n.as_ptr()).val);
                                }
                            }
                        } else {
                            match left {
                                None => {
                                    println!("The left node is None.");
                                    assert!(false);
                                },
                                Some(ln) => {
                                    (*n.as_ptr()).val = (*ln.as_ptr()).val.clone();
                                    (*n.as_ptr()).left = self.remove_with_node(*left, &(*n.as_ptr()).val);
                                }
                            }
                        }
                        self.update_node_height(node);
                        return node;
                    }
                }
            }
        };
        self.update_node_height(node);
        node
    }

    pub fn insert(&mut self, val: T) {
        self.root = self.insert_with_node(self.root, val).clone();
    }

    pub fn remove(&mut self, val: &T) {
        self.root = self.remove_with_node(self.root, val).clone();
    }
}

impl<T> Tree<T> where T: Display + Ord + Clone {
    pub fn print_tree(&self) {
        match &self.root {
            None => {
                println!("Empty tree");
                return ;
            },
            Some(r) => {
                let mut temp = self.root.clone();
                let mut queue: Vec<Option<NonNull<TreeNode<T>>>> = Vec::new();
                queue.push(temp.clone());
                let mut column: u32 = 1;
                let mut count: u32 = 0;
                let max_depth: u32 = unsafe {
                    1 << ((*r.as_ptr()).height - 1)
                };
                while !queue.is_empty() && column <= max_depth {
                    temp = queue.remove(0);
                    count += 1;
                    match temp {
                        None => {
                            print!("null ");
                            queue.push(None);
                            queue.push(None);
                            if count == column {
                                count = 0;
                                column <<= 1;
                                println!("");
                            }
                            continue;
                        },
                        Some(v) => unsafe {
                            print!("{} ", &(*v.as_ptr()).val);
                            match &(*v.as_ptr()).left {
                                None => {
                                    queue.push(None);
                                },
                                Some(ln) => {
                                    queue.push(Some(*ln));
                                }
                            }
                            match &(*v.as_ptr()).right {
                                None => {
                                    queue.push(None);
                                },
                                Some(rn) => {
                                    queue.push(Some(*rn));
                                }
                            }
                            if count == column {
                                count = 0;
                                column <<= 1;
                                println!("");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut tree: Tree<i32> = Tree::new();
    let nums = vec![4,6,8,2,1,5,7,9];
    for i in nums {
        println!("insert {}", &i);
        tree.insert(i);
        println!("");
    }
    tree.print_tree();
    tree.remove(&4);
    tree.print_tree();
}
