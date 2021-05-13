/**********************************************
  > File Name		: own.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue 16 Feb 2021 04:02:06 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

fn func(mut t: i32) {
    println!("{}", t);
    t += 2;
}

struct S {
    i: i32
}

fn main() {
    let mut v = vec![S {i: 32}, S {i: 23}];
    let mut f = vec![S {i: 45}];
    let d = S{i:11};
    let k = d;
    println!("{}", v[1].i);
}
