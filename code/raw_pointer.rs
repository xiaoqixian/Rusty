/**********************************************
  > File Name		: raw_pointer.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat 13 Feb 2021 11:31:19 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn main() {
    let mut d = 5;
    let mut f = 6;
    let r2 = &mut f as *mut i32;
    let r1 = &mut d as *mut i32;
    println!("{:?}", r1);
    println!("{:?}", r2);
    unsafe {
        println!("{}", *r1);
        *r2 += 2;
        println!("{}", *r2);
    }
}
