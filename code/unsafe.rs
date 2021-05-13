/**********************************************
  > File Name		: unsafe.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat 13 Feb 2021 03:21:10 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    let d = 5;
    use std::rc::Rc;
    let f = Rc::new(&d);
    println!("{}", f);
}
