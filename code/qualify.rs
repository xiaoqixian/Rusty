/**********************************************
  > File Name		: qualify.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Fri 19 Feb 2021 12:40:35 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up!");
    }
}

//impl Human {
    //fn fly(&self) {
        //println!("*waving arms furiously*");
    //}
//}

fn main() {
    let h = Human{};
    Pilot::fly(&h);
}

