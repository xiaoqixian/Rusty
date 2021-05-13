/**********************************************
  > File Name		: ge_de.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 15 Feb 2021 03:35:59 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

/*
 * test generics default type
 */

struct D {
    s: u32
}

struct S<T, K> {
    t: T,
    k: K,
}

impl<T> S<T, D> {
    fn print() {
        println!("woc");
    }
}

fn main() {
    let s: S<i32, D> = S {
        t: 32,
        k: D {
            s: 23,
        },
    };
    s.print();
}
