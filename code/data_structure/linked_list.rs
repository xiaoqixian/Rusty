/**********************************************
  > File Name		: linked_list.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 15 Feb 2021 02:53:05 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

/*
 * Write a linked list with Rust
 */

use std::rc::Rc;
use std::boxed::Box;

enum Node<T> {
    Conn(T, Rc<Box<Node>>),
    Nil,
}

struct List<'a, T> {
    head: Option<&Node>,
    tail: Option<&Node>,
}

impl<T> List<'a, T> {
    fn new() -> List {
        List {
            head: &'a Node::Nil,
            tail: &'a Node::Nil,
        }
    }

    fn add(&mut self, val: T) {
        let mut new = Node::Conn(val, Rc::new(Node::Nil));
        match self.head {
            Node::Nil => {
                self.head = &new;
                return;
            },
            Node::Conn(_,_) => {
                match self.tail {
                    Node::Nil => {
                        self.tail = &new;
                    },
                    Node::Conn(v,_) => {
                        let mut temp_tail = Node::Conn(v, Rc::clone(&new));
                        self.tail = &new;
                    }
                }
            }
        }
    }
}

fn main() {

}
