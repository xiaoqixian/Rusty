/**********************************************
  > File Name		: unwrap.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed 24 Feb 2021 09:30:02 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

struct AS {
    i: i32
}

impl Drop for AS {
    fn drop(&mut self) {
        println!("AS i={} dropped", self.i);
    }
}

fn main() {
    let s = Box::new(String::from("Hello"));
    let mut p = Box::new(AS {i:32});
    let k = &p;
    {
        p = Box::new(AS {i: 43});
    }
    println!("{}", k.i);
}
