/**********************************************
  > File Name		: stack.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Fri 29 Jan 2021 12:08:20 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

/*
 * implement a stack with Rust array and generics
 */

use std::cell::RefCell;

struct Stack<T> {
    stack: RefCell<Vec<T>>,
}

use std::fmt::Display;
impl<T: Display> Stack<T> {
    fn new() -> Self {
        Stack {
            stack: RefCell::new(vec![])
        }
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }

    fn push(&self, item: T) {
        self.stack.borrow_mut().push(item);
    }

    fn is_empty(&self) -> bool {
        self.stack.borrow().is_empty()
    }

    fn size(&self) -> usize {
        self.stack.borrow().len()
    }

    fn top(&self) -> Option<&T> {
        self.stack.into_inner().last()
    }

    fn display(&self) {
        print!("[", );
        for i in self.stack.borrow().iter() {
            print!("{}, ", i);
        }
        println!("]");
    }
}

fn main() {
    let s: Stack<i32>  = Stack::new();
    s.push(4);
    s.push(6);
    s.pop();
    s.display();
}
