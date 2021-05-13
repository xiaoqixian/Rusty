## 关于Rust raw pointers的引用返回问题

我们知道在rust中，raw pointers解引用之后可以不受borrowing rules的限制，可以进行多次 mutable 的引用。但是这个仅限制于当前 scope 之内，如果你想要返回这个引用，就可以收到编译错误。提示你在返回一个在当前函数中产生的数据的引用。

比如下面的这段代码：

```rust
use std::ptr::NonNull;
struct S {
    i: i32
}

impl Clone for S {
    fn clone(&self) -> Self {
        S {
            i: self.i
        }
    }
}

struct F {
    v: Vec<NonNull<S>>
}

impl F {
    pub fn func<'a>(&'a self) -> &'a mut S {
        let s = &mut unsafe {
            *self.v[0].as_ptr()
        };
        s
    }
}


fn main() {
    let f = F {
        v: vec![NonNull::new_unchecked(Box::into_raw(Box::new(S{i: 32}.clone()))); 5]
    };
    f.func();
}
```

按道理来说，如果一个数据可以被多次mutable引用的话，那么它被多次返回一个mutable引用应该也是没有问题的。但是编译器会提示你在返回一个在当前函数栈中的数据引用。