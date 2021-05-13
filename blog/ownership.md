---
author: lunar
date: 2021年01月11日 星期一 19时00分39秒
location: Shanghai
---

## Ownership

### Reference and Borrowing

In Rust, we use sign `&` to refer a variable, so as in C++.

Reference can be used as a constant parameter passed to functions, which means it is not allowed to change the original value of the parameter caused it's borrowed from another variable.

If you want to change the value, use mutable reference: `&mut`.

But mutable reference has a big restriction: **only one mutable reference of a particular piece of data is allowed in a particular scope**.

It's pretty useful when there is no mechanism being used to synchronize access to the data.

Also, when there is an immutable reference to a variable, any mutable references are not allowed either. Obviously user won't expect any mutable references when there is an immutable reference. Otherwise, the behavior of the immutable reference will be unexpected.

>   Note that a reference's scope from where it is introduced and continues through the last time that reference is used.

According to the sentence, the following case is OK.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

###### Dangling Reference

If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

#### String Slice Type

A *string slice* is a reference to part of a string, we can create slices using a range within brackets by specifying `[stringing_index..ending_index]`.

