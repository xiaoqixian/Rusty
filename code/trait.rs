/**********************************************
  > File Name		: trait.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Thu 28 Jan 2021 09:26:29 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn name(&self) {
        println!("I am a shape");
    }
    fn print_area(&self) {
        println!("The area of the shape: {}", self.area());
    }
    fn print_perimeter(&self) {
        println!("The perimeter of the shape: {}", self.perimeter());
    }
}

struct Rectangle {
    length: f32,
    width: f32,
}

struct Circle {
    radius: f32
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self) -> f32 {
        self.length + self.width
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }
    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}

fn exhibit(shape: &impl Shape) {
    println!("Exhibit a shape");
    shape.print_area();
    shape.print_perimeter();
}

fn display<T>(shape: &T) where T: Shape {

}

fn main() {
    let rec = Rectangle {
        length: 12.0,
        width: 6.0,
    };
    exhibit(&rec);
}
