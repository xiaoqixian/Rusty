/**********************************************
  > File Name		: match.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed 03 Mar 2021 08:36:37 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

enum S {
    Okay,
    T,
    Error,
}

fn main() {
    let s = S::T;
    if let S::Okay != s {
        println!("suck");
    }
}
