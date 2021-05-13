/**********************************************
  > File Name		: rc.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 15 Feb 2021 09:31:54 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    use std::rc::Rc;
    let mut d = 5;
    let mut p = 6;
    let mut r = Rc::new(&mut d);
    let m = match Rc::get_mut(&mut r) {
        Some(m) => m,
    };
    *m += 5;
    println!("{}", r);
}
