/**********************************************
  > File Name		: handle.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Thu 04 Mar 2021 04:11:51 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/
use std::io;
use std::io::prelude::*;
use std::io::File;
use same_file::Handle;

fn main() {
    let f1 = File::open("alloc.rs");
    let f2 = File::open("alloc.rs");
    let h1 = Handle::from_file(f1);
    let h2 = Handle::from_file(f2);
    if h1 == h2 {
        println!("from the same file");
    }
}
