/**********************************************
  > File Name		: alloc.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 01 Mar 2021 08:06:48 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    let mut s: Box<[i32]>;
    s = Box::new([0; 20]);
    println!("{}", s.len());
}
