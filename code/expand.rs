/**********************************************
  > File Name		: expand.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed 03 Mar 2021 11:37:46 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/


fn main() {
    let mut s = vec![0; 5];
    s.resize(10, 0);
    println!("{}", s.len());
}
