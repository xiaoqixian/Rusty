# Advanced Features

### Unsafe Rust

So far, you can see that Rust has a strict memory safety guarantee at compile time. However, sometimes this safety guarantee can be annoying (or to say for the most of the time).

Therefore, Rust has a second language hidden inside that doesn't enforce these memory safety guarantees: it's called *unsafe* Rust and works just like regular Rust, but gives us extra superpowers.

When the compiler tries to tell whether or not the code upholds the memory safety rules, it's better to reject some valid programs than accept some invalid programs. This results to that sometimes your code may be OK for you, but won't compile. In these cases, you can use the `unsafe` block to tell the compiler that you can guarantee that your code is acceptable. 

Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe. If Rust didn't let you do unsafe operations, you couldn't do certain tasks. 

#### Unsafe Superpowers

You can take five operations in unsafe Rust, called *unsafe superpowers*, that you can't in safe Rust.

-   Dereference a raw pointer.
-   Call an unsafe function or method.
-   Access or modify a mutable static variable.
-   Implement an unsafe trait.
-   Access fields of `union`s

Note that `unsafe` **doesn't turn off the borrow checker**, the `unsafe` keyword only gives you access to these five features. 

#### Dereferencing a Raw Pointer

Unsafe Rust has two new types called *raw pointers* that are similar to references. As with references, raw pointers can be mutable or immutable,  written as `*mut T` and `*const T`, immutable means that the pointer can't be directly assigned to after being dereferenced. 

Different from references and smart pointers, raw poitners:

-   Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location.
-   Aren't guarantee to point to valid memory.
-   Are allowed to be null.
-   **Don't implement any automatic cleanup.**

To create an immutable and a mutable raw pointer from a reference:

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

We don't use the unsafe block, cause we can define raw pointers in safe code, **we just can't dereference raw pointers in safe code.** 

To dereference a raw pointer, we have to add a unsafe block. 

```rust
unsafe {
    println!("{}", *r1);
    *r2 += 2;
    println!("{}", *r2);
}
```

Note that if we instead use references, the code won't compile. Cause we create an immutable reference and a mutable reference at the same scope, which breaks the Rust's ownership rules. However, with raw pointers, we don't have to follow the rules, which may cause potential data races. So you have to be careful with raw pointers.

#### Calling an Unsafe Function or Method

A unsafe function or method are denoted with a `unsafe` keyword. The `unsafe` keyword indicates that the function has requirements that we need to uphold when we call the function. By calling the function within a `unsafe` block, we're saying that we know that the function we're calling is unsafe and we take the responsibility for upholding the function's contracts. On the other hand, if we call them without the unsafe block, we'll get compile error.

#### Creating a Safe Abstraction over Unsafe Code

Just because a function contains unsafe code doesn't mean that we have to mark the entire function as unsafe. In fact, wrapping unsafe code in a safe function is a common abstraction.

Also, we don't need to mark the function as `unsafe`, and we can call the function from safe Rust. As the unsafe block already create a safe abstraction to the unsafe code.

#### Using `extern` Functions to Call External Code

Sometimes, your Rust code might need to interact with code written in another language. Rust prepared a keyword `extern`, that facilitates the creation and use of a *foreign function interface (FFI)*.

Note that functions declared within the `extern` block is always unsafe because other languages don't enforce Rust's rules and guarantees. 

```rust
#[line(name="c_lib_name")]
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

The `#[link(...)]` annotation is to instruct the linker to link against the C library so the function symbols are resolved.

Within the `extern "C"` block, we list the names and signatures of external functions from another language we want to call. The `"C"` part defines which *application binary interface (ABI)* the external function uses: the ABI defines how to call the function at the assembly level. The `"C"` ABI is the most common and follows the C programming language's ABI.

#### Calling Rust Functions from Other Languages

We can also use `extern` to create an interface that allows other languages to call Rust functions. 

```rust
#![allow(unused)]
fn main() {
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
}
```

Instead of using an `extern` block, we add the `extern` keyword and specify the ABI to use just before the `fn` keyword. 

We also use the `#[no_mangle]` annotation to tell the compiler that not to mangle the name of this function. *Mangle* is when compiler compile your code, it slightly changes your functions' names to contain more information but less human-readable. 

#### Accessing or Modifying a Mutable Static Variable

Static variables are similar to constants. The names of static variables are all in uppercase by convention, and we must annotate the variable's type. Static variables can only store references with the `'static` lifetime, which means the Rust compiler can figure out the lifetime; we don't need to annotate it explicitly. 

Accessing an immutable static variable is safe.

A subtle difference between static variables and constants is that static variables have fixed memory addresses to store their data; while constants, on the other hand, are allowed to duplicate their data whenever they're used. In my guessing, the Rust compiler may replace constants with their data in compile time. 

Another difference is that static variables can be mutable. **And accessing and modifying mutable static variables is unsafe.**

```rust
//to declare an immutable static variable
static HELL_WORLD: &str = "Hello World";
//to declare a mutable static variable
static mut NUMBER: u32 = 0;
```

Any code **reads or writes** static mutable variables must be within an unsafe block. 

#### Implementing an Unsafe Trait

We can declare a trait as unsafe by adding the `unsafe` keyword before the `trait` keyword and **marking the implementation of the trait as unsafe too**.

```rust
unsafe trait Func {
    //methods
}

unsafe impl Func for i32 {
    //method implementation go here
}
```

### Advanced Traits

#### Specifying Placeholder Types in Trait Definitions with Associated Types

*Associated types* connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used in this type's place for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented. 

One example with an associated type is `Iterator` trait.

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Implement the trait and specify the associated type. 

```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        //
    }
}
```

It's kind of similar to generics:

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference is that when using generics, we must annotate the types in each implementation; because we can also implement `Iterator<String> for Counter` or any other type. When we use the `next` method on `Counter`, we have to provide type annotations to indicate which implementation of `Iterator` we want to use.

With associated types, we can only implement a trait on a type once. But we don't have to specify the type.

So if you want to implement a trait for different types differently, you need to use generics. Otherwise, for simplicity, you can use associated types.

#### Default Generic Type Parameters and Operator Overloading

Rust allows you to overload the operations and corresponding traits in `std::ops` by implementing the traits associated with the operator. 

For example, if we want implement the `+` parameter for a type. We need to implement the `Add` trait and implement the `add` method inside the trait.

```rust
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x;
            y: self.y + other.y;
        }
    }
}
```

We can also set a default generic type in a trait. For example:

```rust
pub trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::output;
}
```

The `Rhs=Self` syntax is called *default type parameter*. The `Rhs` defines the type of parameter `rhs`. If we don't specify a concrete type for `Rhs` when we implement the `Add` trait, the type of `Rhs` will default to `Self`, which will be the type we're implementing `Add` on.

#### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as another trait's method, nor does Rust prevent you from implementing both traits on one type. It's also possible to implement a method directly on the type with the same name as methods from traits.

```rust
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
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    
}
```

In the above code, we implement many `fly` methods for human, and every one is implemented differently. 

When we call `fly` on an instance of `Human`, the compiler defaults to calling the method that is directly implemented on the type. 

If we want to call methods implemented from other traits, we have to call them this way:

```rust
let h = Human{};
Pilot::fly(&h);
Wizard::fly(&h);
```

When we have multiple similar associated functions defined on a type, we need to use `Type_name::` or `<Type_name as Trait_name>::` as a prefix to call the function. The former is for an associated function implemented directly on the type and the latter is for an associated function implemented from a trait. 

For example, when we implement a type `Dog` and define an associated function `new`, as well as we implement a trait `Animal` on `Dog` and implement the associated function `new` in `Animal`.

To differ this two functions, we need to call them as `Dog::new()` and `<Dog as Animal>::new()` (Assume the function doesn't accept any parameters.).

#### Using Supertraits to Require One Trait's Functionality Within Another Trait

Sometimes, you might need one trait to use another trait's functionality. In this case, you need to rely on the dependent trait also being implemented. The trait you rely on is a *supertrait* of the trait you're implementing.

For example, when we need to define a trait `FancyPrint` that will print some fancy things, first of all, the thing need to implement the `Display` trait. Otherwise the trait we define won't work. This is the case when a trait need to rely on another trait. 

To achieve that, we need to explicitly tell the compiler that the trait relies on the `Display` trait. 

```rust
use std::fmt;
trait FancyPrint: fmt::Display {
    
}
```

Also, when a trait has a supertrait, it is able to use any methods defined in the supertrait. Such as `to_string`, any type implements the `Display` trait has a method called `to_string` and we can call it.

#### Using the Newtype Pattern to Implement External Traits on External Types

We have ever talked about the *orphan rule* that states we're allowed to implement a trait on a type as long as either the trait or the type are local to our crate. It's possible to get around this restriction using the *newtype pattern*, which involves creating a new type in a tuple struct.

The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for.

As an example, let's say we want to implement `Disply` on `Vec<T>`, which breaks the orphan rule because neither `Display` nor `Vec<T>` is local to our crate. 

But we can define a `Wrapper` struct that holds an instance of `Vec<T>`. Then we can implement `Display` on `Wrapper` and use the `Vec<T>` value.

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) => fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

### Advanced Types

#### Creating Type Synonyms with Type Aliases

Rust provides the ability to declare a *type alias* to give an existing type another name. This feature is similar to `typedef` keyword in C/C++.

With syntax:

```rust
type new_type_name = Type;
```

#### The Never Type that Never Returns

Rust has a special type named `!` that's known in type theory lingo as the *empty type* because it has no values. We prefer to call it the *never type* because it stands in the place of the return type when a function will never return.

```rust
fn bar() -> ! {
    
}
```

The never type is useful when it comes to `match` expression, as mentioned before, all branches of a match expression have to return a same type value when it has to return a value. But what if we don't want to return a value in one of the branches? We return a `!` type value or should I say a never type. Then the compiler won't take the type of the return value of the branch into consideration. So we can use `continue` or `panic!` in such branch.

#### Dynamically Sized Types and the `Sized` Trait

The dynamically sized types are types whose size can be known only at runtime.

To work with dynamically sized types, Rust has a particular trait called the `Sized` trait to determine whether or not a type's size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time.

However, you can use following special syntax to relax this restriction:

```rust
fn generic<T: ?Sized>(t: &T) {
    
}
```

As the size of type `T` might be unknown, so the type of parameter `t` has to be `&T`.

### Advanced Functions and Closures

#### Function Pointers

We can pass regular functions to functions, just need to name the type of the function parameter as the function signature you want to pass in.

For example:

```rust
fn use_function(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the parameter type directly rather than declaring a generic type parameter with one of the `Fn` traits as a trait bound.

#### Returning Closures

Closures are represented by traits, which means you can't return closures directly. 

```rust
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x+1)
}
```

### Macros 

We've used plenty of macros like `print!` or `panic!`, but we haven't fully explored what a macro is and how it works. 

#### The Difference Between Macros and Functions

Fundamentally, macros are a way of writing code that writes other code, which is known as *metaprogramming*. All of the macros *expand* to produce more code than the code you've written manually. 

A function signature must declare the number and type of code you have to write and maintain. Macros, on the other hand, can take a variable number of parameters. 

Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

#### Declarative Macros with `macro_rules!` for General Metaprogramming

To define a macro, you use the `macro_rules!` construct. Let's explore how to use `macro_rules!` by looking at how the `vec!` macro is defined.

```rust
#[macro_export]
macro_rules! vec {
    ($($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
            	temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The `#[macro_export]` annotation indicates that **this macro should be made available whenever the crate in which the macro is defined is brought into scope.**

We then start the macro definition with `marco_rules!` and the name of the macro we're defining without the exclamation mark. 

The next code block is similar to `match` expression, the `($($x: expr))` is a pattern which may be far away from easy-understanding. Given that this is the only pattern in this macro, there is only one valid way to match; any other pattern will result in an error. 

Let's walk through the valid pattern syntax in macro definitions.

First, a set of parentheses encompasses the whole pattern. A dollar sign (`$`) is next, followed by a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code. Within `$()` is `$x:expr`, which matches any Rust expression and gives the expression the name `$x`.

The comma following `$()` indicates that a literal comma separator character could optionally appear after the code that indicates the code in `$()`. The `*` specifies that the pattern matches zero or more of whatever precedes the `*`.

#### Procedural Macros for Generating Code from Attributes

