/**********************************************
  > File Name		: enum.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 25 Jan 2021 03:12:55 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

//enum test

enum Message {
    quit,
    pwd(String),
}

fn main() {
    let mut o = &Message::quit;
    let q = Message::pwd(String::from("Damn"));
    match q {
        Message::quit => {
            println!("quit");
        }
        Message::pwd(v) => {
            println!(v);
        }
    }
}
