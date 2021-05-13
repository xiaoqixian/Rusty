/**********************************************
  > File Name		: take.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue 16 Feb 2021 05:48:22 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::boxed::Box;

struct DS {
    i: i32
}

impl DS {
    fn traverse(&mut self) {
        if self.i == 0 {
            return ;
        }
        else {
            println!("i = {}", self.i);
            self.i -= 1;
            self.traverse();
        }
    }
}

fn func(i: &mut i32) -> &mut Option<i32> {
    let sd = Box::new(Some(32));
    unsafe {&mut(*Box::into_raw(sd))
}}

fn f() -> Option<i32>{
    let l = &Some(32);
    return *l;
}

fn main() {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    h.insert(1, DS {i: 2});
}
