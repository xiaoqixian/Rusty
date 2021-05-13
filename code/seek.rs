/**********************************************
  > File Name		: seek.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue 02 Mar 2021 09:44:42 AM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("temp.txt").unwrap();
    let mut buffer: [u8; 5] = [0; 5];
    f.read(&mut buffer);
    for c in buffer.iter() {
        print!("{}", c);
    }

    f.seek(SeekFrom::Start(0));
    f.read(&mut buffer);
    for c in buffer.iter() {
        print!("{}", c);
    }

}
