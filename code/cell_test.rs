/**********************************************
  > File Name		: cell_test.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Fri 12 Feb 2021 09:52:01 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    use std::cell::RefCell;
    let c = RefCell::new(5);
    let mut d = c.borrow_mut();
    *d = 6;
    println!("{:?}", c);
}
