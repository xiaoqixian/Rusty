#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn length(&self) -> i32 {
        self.width + self.height
    }
    fn change_width(&mut self, width: i32) {
        self.width = width;
    }
}


fn main() {
    let mut rect = Rectangle {
        width: 2,
        height: 3,
    };
    println!("Rectangle area: {}", rect.area());
    println!("Rectangle length: {}", rect.length());
    rect.change_width(5);
    println!("Rectangle area: {}", rect.area());
}
