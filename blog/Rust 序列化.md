## Rust 序列化

将Rust中的 struct 与 vector 的数据序列化到文件中，然后再读取出来。

```rust
use std::ptr::NonNull;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::FileExt;

#[derive(Debug, Copy, Clone)]
struct S {
    i: usize,
    j: usize,
}

fn write() {
    let v = Box::new(S {
        i: 0,
        j: 0,
    });
    let data: Vec<u8> = vec![10; 128];
    let f = OpenOptions::new().read(true).write(true).create(true).open("seria").unwrap();
    println!("{}", std::mem::size_of::<S>());
    unsafe {
        let sli = std::slice::from_raw_parts(Box::into_raw(v) as *const u8, std::mem::size_of::<S>());
        f.write_at(sli, 0);
        f.write_at(&data[..], std::mem::size_of::<S>() as u64);
    }
}

fn read() {
    let mut v = S{i:12, j:12};
    let mut data: [u8; 128] = [0; 128];
    let f = File::open("seria").unwrap();
    unsafe {
        let sli = std::slice::from_raw_parts_mut(&mut v as *mut _ as *mut u8, std::mem::size_of::<S>());
        f.read_at(sli, 0);
        f.read_at(&mut data, std::mem::size_of::<S>() as u64);
        v = *(sli.as_ptr() as *const S);
        println!("{:?}", v);
        println!("{:?}", data);
    }
}

fn main() {
    write();
    read();
}
```

