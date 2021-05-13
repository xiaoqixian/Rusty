/**********************************************
  > File Name		: exchange.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue 16 Feb 2021 05:31:47 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    let a = 1;
    let b = 4;
    let temp = &a;
    let a = b;
    let b = *temp;
    println!("{}, {}", a, b);
}
