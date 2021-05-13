# Smart Pointers

A *pointer* is a general concept for a variable contains an address in memory.

*Smart pointers* are data structures that not only act like a pointer but also have additional metadata and capabilities. 

The concept of smart pointer is usually implemented by using structs. The characteristic that distinguishes smart pointers from ordinary structs is that smart pointers implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of a smart pointer to behave like a reference so you can write code to work with either references or smart pointers. The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of its scope. 

### Using `Box<T>` to Point to Data on the Heap

The `Box<T>` type allows you to store data on the heap rather than on the stack.

#### Using `Box<T>` to allocate memory on the heap

```rust
fn main() {
    let b = Box::new(5);
}
```

When `b` goes out of the scope, it will be deallocated at the end of `main`, the deallocation happens for the box and the data it points to.

### Treating Smart Pointers Like Regular References with the `Deref` Trait.

Implementing the `Deref` trait allows you to customize the behavior of the *dereference operator*, `*`. 

When we want to access the data that a pointer points to, we use the dereference operator `*` to achieve it. 

```rust
fn main() {
    let x = 5;
    let y = &x;
    println("{}", *y);
}
```

#### Defining Our Own Smart Pointer

The `Box<T>` type is ultimately defined as a tuple struct with one element. 

So we'll define a smart pointer just like it.

```Rust
struct MyPointer<T>(T);

impl<T> MyPointer<T> {
    fn new(x: T) -> MyPointer<T> {
        MyPointer(x)
    }
}
```

#### Treating a Type Like a Reference by Implementing the `Deref` Trait

```rust
use std::ops::Deref;

impl<T> Deref for MyPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
```

By `&self.0` the `Deref` method returns a reference to the value we want to access with the `*` operator.

Without the `Deref` trait, the compiler can only dereference `&` references. The `deref` method gives the compiler the ability to make a value of any type that implements `Deref` and call the `deref` method to get a `&` reference that it knows how to dereference. 

#### Implicit  Deref Coercions with Functions and Methods

Deref coercion is a convenience that Rust performs on arguments to functions and methods. It only works on types that implement the `Deref` trait. Deref coercion happens automatically when we pass a reference whose type does not match the type of the argument to a function or a method. 

#### How Deref Coercion Interacts with Mutability

Similar to how you use the `Deref` trait to override the `*` operator on immutable references, you can use the `DerefMut` trait to override the `*` operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

-   From `&T` to `&U` when `T:Deref<Target=U>`
-   From `&mut T` to `&mut U` when `T:DerefMut<Target=U>`
-   From `&mut T` to `&U` when `T:Deref<Target=U>`

### Running Code on Cleanup with the `Drop` Trait

The `Drop` trait allows you to customize what happens when a value goes out of its scope. 

You can provide an implementation for the `Drop` trait on any type, and the code you specify can be used to release resources.

To implement the `Drop` trait, you don't need to do some extra things about memory cleanup cause the compiler will automatically insert the memory cleanup code, so there will never have memory leak. 

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

#### Dropping a Value Early with `std::mem::drop`

Rust doesn't let you call the `Drop` trait's `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.

The `std::mem::drop` is a public API provided by the Rust standard library. When you use the API, you pass the value that you want to force to be dropped as an argument. 

You don't have to worry about problems resulting from accidentally cleaning up values still in use: the ownership system that makes sure references are always valid also ensures that `drop` gets called only once when the value is no longer being used.

### `Rc<T>`, the Reference Counted Smart Pointer

There are chances that a single value might have multiple owners. For these cases, Rust has the `Rc<T>` type to keep track of the number of references to a value which determines whether or not a value is still in use. The `Rc` is an abbreviation for *reference counting*.

Note that `Rc<T>` is only for use in single-threaded scenarios. When we discuss concurrency later, we'll cover how to do reference counting in multithreaded programs.

#### Creating a Reference Counted Smart Pointer and Increase References

To create a new counted reference smart pointer:

```rust
fn main() {
    let a = Rc::new(String::from("Suck"));
}
```

To increase a reference:

```rust
fn main() {
    let b = Rc::clone(&a);
}
```

Unlike many her languages, the `clone` method is not a deep clone, it does make a deep copy of all the data, instead, it only increments the reference count. 

We could have called `a.clone()` rather than `Rc::clone(&a)`, but Rust's convention is to use `Rc::clone` in this case.

### `RefCell<T>` and the Interior Mutability Pattern

*Interior mutability* is a design pattern that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing. We will talk about unsafe code later.

#### Enforcing Borrowing Rules at Runtime with `RefCell<T>`

With references and `Box<T>`, the borrowing rule's invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced at runtime. With references, if you break the borrowing rules, you will get compile errors; with `RefCell<T>`, if you break the borrowing rules, your program will panic and exit.

The `RefCell<T>` is useful when you're sure that your code follows the borrowing rules but the compiler is unable to guarantee that.

Differences between `Box<T>`, `Rc<T>`, and `RefCell<T>`:

-   `Rc<T>` enables multiple owners of the same data; while `Box<T>` and `RefCell<T>` have single owners.
-   `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only **immutable** borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime. 
-   Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable. 

#### Interior Mutability: A Mutable Borrow to an Immutable Value

Usually, an immutable value cannot have a mutable reference. However, there are cases that we hope an immutable value to be mutable in some certain methods or functions. 

For instance, say we want to create a stack object with the vector provided by Rust.

```Rust
struct Stack {
    array: vec<i32>,
    top: i32,
}
```

For a stack, we need to insert values into it and pop values out of it. So we implement some method for it.

```rust
impl Stack {
    fn push(&self, val: i32) {
        self.array.push(val);
    }
    
    fn pop(&self) -> Option<i32> {
        self.array.pop();
    }
}
```

If we compile the above code, we will get a compile error. As the parameter `&self` is an immutable parameter, but we mutate it in both methods. That is not allowed, so need to change the immutable reference to mutable reference. Even though it may accidentally mutate other fields, like `top`, of `self` too. 

However, with `RefCell<T>`, we have a better choice. We can make wrap the array up with the `RefCell<T>`. Then the `array` field will be mutable even though the `self` reference is immutable.

```rust
struct Stack {
    array: RefCell<vec<i32>>,
    top: i32,
}

impl Stack {
    fn new(&self) -> Stack {
        Stack {
            array: RefCell::new(vec![]);
            top: -1;//-1 means no value in the stack yet
        }
    }
    fn push(&self, val: i32) {
        self.array.borrow_mut().push(val);
    }
    
    fn pop(&self) -> Option<i32> {
        self.array.borrow_mut().pop();
    }
}
```

Note that when we want to refer the `RefCell<T>` mutably, we need to call the method `borrow_mut()`, otherwise we call the method `borrow()`. The `borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we can treat them like regular references.

#### Keeping Track of Borrows at Runtime with `RefCell<T>`

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. 

If we try to violate the borrowing rules, rather than getting a compile error as we would with references, the implementation of `RefCell<T>` will panic at runtime. 

#### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

`Rc<T>` gives us a chance to have multiple owners of some data, but all of them have to be immutable. If we want to get a value that can have multiple owners and we can mutate them, we need a `Rc<T>` that holds a `RefCell<T>`.

#### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

We've known that calling `Rc::clone` increases the `strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned up if its `strong_count` is 0. 

You can also create a *weak reference* to the value within an `Rc<T>` by calling `Rc::downgrade` and passing a reference to the `Rc<T>`. When you call `Rc::downgrade`, you get a `Weak<T>`, and instead of increasing the `strong_count` of an `Rc<T>` instance, you have a `weak_count` increased by 1. The `Rc<T>` instance uses `weak_count` to keep track of how many weak references exist. Most importantly, the `Ref<T>` instance doesn't need the `weak_count` to be 0 to get cleaned up. 

As a value that a `Weak<T>` instance points to might be already dropped. So every time we use a `Weak<T>` instance, we have to make sure that the value it points to still exist. We can achieve that by calling the `upgrad` method which return a `Option<Rc<T>>` type value. If the result is None, the value is dropped. If the result is Some, we can still use the value. 