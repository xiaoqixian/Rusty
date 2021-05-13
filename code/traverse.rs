/* A traverse attemtation*/
fn traverse(func: Fn(), i: i32) {
    if i != 0 {
        println!("{}", i);
        func(func, i-1);
    }
}

fn main() {
    traverse(traverse, 5);
}
