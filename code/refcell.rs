/**********************************************
  > File Name		: refcell.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat 13 Feb 2021 01:25:50 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let a = Rc::new(RefCell::new(5));
    let b = Rc::clone(&a);
    *(b.borrow_mut()) += 2;
    *(a.borrow_mut()) += 2;
    println!("{:?}", a);
}
