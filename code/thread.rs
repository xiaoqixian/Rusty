/**********************************************
  > File Name		: thread.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue 23 Feb 2021 04:00:26 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hello Receiving");
        tx.send(&val).unwrap();
        println!("Sending End: {}", val);
    });
    let received = rx.recv().unwrap();
    println!("I received \"{}\"", received);
}
