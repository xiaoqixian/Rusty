/**********************************************
  > File Name		: read.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Fri 26 Feb 2021 02:38:55 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fs::File;
use std::io;
use std::io::prelude::*;

struct wrapper(u8);

impl Clone for wrapper {
    fn clone(&self) -> Self {
        println!("clone");
        wrapper(self.0)
    }
}
impl Drop for wrapper {
    fn drop(&mut self) {
        println!("u8 is dropped");
    }
}

fn d() -> Vec<wrapper> {
    let v = vec![wrapper(5); 8];
    v
}


fn func() -> Vec<u8> {
    let mut fp = File::open("temp.txt").unwrap();
    let mut v: Vec<u8> = vec![0; 8];
    fp.read(v.as_mut_slice());
    let mut p: [u8; 8] = [5; 8];
    fp.seek(io::SeekFrom::Start(0));
    fp.read(&mut p);
    println!("{:?}", v);
    println!("{:?}", p);
    v
}

struct F {
    v: Vec<wrapper>
}

fn main() {
    let mut res = d();
    println!("Not dropped yet");
    println!("{}", res[0].0);
    //set memory
    unsafe {
        let ptr = res.as_mut_ptr();
        std::ptr::write_bytes(ptr, 0x00, 4);
    }
    println!("{}", res[0].0);
    println!("{}", res[4].0);
}
