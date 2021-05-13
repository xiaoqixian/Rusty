/**********************************************
  > File Name		: chars.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat 27 Feb 2021 04:48:17 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    let s = String::from("hello");
    let c: Vec<char> = s.chars().collect();
    println!("{}", c[1] as u32 - 'd' as u32);
    println!("{}", max!(3,4));
}
