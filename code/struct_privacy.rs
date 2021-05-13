/*
 * test struct privacy
 */

//in this case, struct is all free to access
struct rectangle {
    length: i32,
    width: i32,
}

mod shape {
pub struct Rectangle {
    pub length: i32,
    width: i32,
}
}

fn main() {
    let mut rec = rectangle {
        length: 32,
        width: 24,
    };
    rec.length = 45;
}
