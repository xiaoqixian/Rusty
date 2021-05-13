# Generic Types, Traits, and Lifetimes

In Rust, *generics* is a tool for establishing abstract stand-ins for concrete types or other properties. When we're writing code, we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

### Generic Data Types

Let's first look at how to define functions, structs, enums, and methods using generics.

#### In Function Definitions

Let's define a function that finds the largest element in a vector for example. To parameterize the types in the new function we'll define, we need to name the type parameter. Mostly we use a special identifier to tell the programmer that this is a general type, which is `T` by convention.

Then we have to declare the parameter name in the signature so the compiler knows what the name means. In Rust, the way to achieving that is  place the type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list, like this:

```rust
fn largest<T>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

The `largest` function uses list of general type `T` as an parameter, but this code won't compile cause we don't have set any constriction on the type `T` and not all possible types values can compare with each other. To enable comparisons, the standard library has the `std::cmp::PartialOrd` trait that you can implement on types.

#### In Struct Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

If we want to use multiple generics in one struct, here is the way:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

#### In Enum Definitions

The most typical enum using generics:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Multiple generics in one enum:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

Note that we have to declare `T` just after `impl` so we can use it to specify that we're implementing methods on the type `Point<T>`.

If we want to implement methods on a specific `Point<T>` instance like `Point<i32>`, then we don't need to declare any types after `impl`.

Generic type parameters in a struct definition aren't always the same as those you use in that struct's method signatures. For example:

```rust
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

#### Performance of Code Using Generics

Say first conclusion: **Rust code that implements generics doesn't run any slower than code that implements concrete types.**

You may wonder how Rust accomplish this. It is that Rust accomplishes this by performing monomorphization. *Monomorphization* is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

### Traits: Defining Shared Behavior

A *trait* tells the Rust compiler about functionality a particular type has and can share with other types.

`trait` is like the `interface` in Java.

#### Defining a Trait

Let's say we need to define a `Shape` trait, `Shape` defines a group of behaviors that all kinds of shapes have in common. 

```rust
pub trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}
```

`Shape` have two common methods for shapes, `area` is to calculate the area of the shape, and `perimeter` is to calculate the perimeter of the shape.

#### Implementing a Trait on a Type

We define two kinds of `Shape`: `Rectangle` and `Circle`. 

```rust
struct Rectangle {
    length: i32,
    width: i32,
}

struct Circle {
    radius: i32
}
```

Now we implement the `area` method and the `perimeter` method for `Rectangle` and `Circle` respectively. 

```rust
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
        2 * 3.14 * self.radius
    }
}
```

Use them for rectangle instance and circle instance:

```rust
fn main() {
    let rec = Rectangle {
        length: 12.0,
        width: 6.0,
    };
    let c = Circle {
        radius: 6.0
    };
    println!("rectangle area: {}, perimeter: {}", rec.area(), rec.perimeter());
    println!("circle area: {}, perimeter: {}", c.area(), c.perimeter());
}
```

The code can be compiled. 

The trait implementation is not compulsory for a struct. But once a struct implements a trait, all methods defined inside the trait have to be implemented. 

Like this:

```rust
impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }
}
```

This code won't compile.

Another restriction to note with trait implementation is that we can implement a trait on a type only if either the trait or the type is local to our crate. 

For example, we can implement a standard library trait like `Display` on a local type `Rectangle`; or we can implement a local trait `Shape` on a standard library type `Vec` (maybe `vec` don't fit with this trait, but don't worry about little details.). But we can't implement `Display` on `Vec` in the local crate. This rule ensures that other people's code can't break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.

#### Default Implementations

Sometimes, the definition of a function in a trait can keep the same for all the types that implement the trait. So, instead of writing the function code for every type that implements the trait, we define a default implementation inside the trait. 

Then, as we implement the trait on a particular type, we can keep or override each method's default behavior.

To use a default implementation, we specify an empty `impl` block with `impl Shape for Rectangle {}`. If there are multiple methods to implement in the trait, then we just don't implement the methods with default implementations.

Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation. Like this:

```rust
pub trait Shape {
    fn area(&self) -> f32;
    fn print_area(&self) {
        println!("The area of the shape: {}", self.area());
    }
}
```

#### Traits as Parameters

Now we explore how to use traits to define functions that accept many different types.

For example, we define a function to display what a shape can do. So it's parameter can only be types that implement `Shape`, here is the way to define it:

```rust
fn display(shape: &impl Shape) {
    println!("Display a shape");
    shape.print_area();
    shape.print_perimeter();
}
```

###### Trait Bound Syntax

The above `impl Trait` syntax is a actually a syntax candy, it has a equivalent but longer form like this:

```rust
fn display<T: Shape>(shape: &T) {
    ...
}
```

This form is more verbose. 

And sometimes the syntax candy can be actually less convenient, like when you need a bunch of parameters that all implement `Shape`, then you need to add a `&impl Shape` after all of them. As with trait bound syntax, you just need to add a `&T` after each. So in this case, trait bound syntax is more concise.

###### Specifying Multiple Trait Bounds with `+` syntax

We can also specify more than one trait bound. Say we want our shapes to have colors, so we need to implement the `Color` trait as well. 

```rust
pub fn display(shape: &(impl Shape + Color)) {
    ...
}
```

or

```rust
pub fn display<T: Shape + Color>(shape: &T) {
    ...
}
```

###### Clearer Trait Bounds with `where`Clauses

Using too many trait bounds may make functions too hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature.

For

```rust
pub fn display<T: Shape + Color, U: Eat + Sleep>(item: &T, live: &U) {
    ...
}
```

when using `where` clause, it'll be like this:

```rust
pub fn display<T, U>(item: &T, live: &U)
	where T: Shape + Color,
		  U: Eat + Sleep {
    
}
```

#### Returning Types that Implement Traits

```rust
fn produce_shape() -> impl Shape {
    ...
}
```

#### Fixing the `largest` Function with Trait Bounds

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

#### Using Traits Bounds to Conditionally Implement Methods

By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

For example, for a generic type `Pair<T>`:

```rust
struct Pair<T> {
    x: T,
    y: T,
}
```

It can be `Pair<char>`, or `Pair<i32>`, or `Pair<Vec>`. But if we want to implement a `cmp_display` function for the `Pair<T>`, we may want only the types that implement `Display` and `PartialOrd` traits implement the function. So this is implementing methods conditionally.

```rust
impl <T: Display + PartialOrd> Pair<T> {
    ...
}
```

We can also implement a trait for any type that implements another trait.

```rust
impl<T: Display> ToString for T {
    ...
}
```

We implement the `ToString` for all `T` types that implement the `Display` trait.

### Validating References with Lifetimes

We know that every reference in Rust has a *lifetime*, which is the scope for which that reference is valid.

Most of the time, lifetime are implicit and inferred. But we must annotate the lifetimes of references could be related in a few different ways. 

#### Generic Lifetimes in Functions

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This code won't compile. Cause the compiler won't know which reference between `x` and `y` would be returned, so it causes a reference confusion.

#### Lifetime Annotation Syntax

Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. 

Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

The names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short. Most people use the name `'a`. Lifetime parameters are placed after the `&` of a reference.

Let's rewrite the above code with lifetime annotations:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The lifetime annotations indicate that the references `first` and `second` must both live as long as a generic lifetime. The generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`.

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`, so is the return value.

When annotating lifetimes in functions, the annotations go into the function signature, not in the function body. Cause Rust can analyze the code within the function without any help, it's just impossible to figure out the lifetimes of the variables from outside the function.

When we try to put the return reference into a `result`, the lifetime of `result` must be smaller than two parameters'.

#### Lifetime Annotations in Struct Definitions

It's possible for struct to hold references, in this case we need to add a lifetime annotation on every reference in the struct's definition.

```rust
struct Stack<'a, T> {
    size: u32,
    capacity: u32,
    top: &'a mut T,
    array: [T],
}
```

We declare the name of the generic lifetime parameter inside the angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of `Stack` can't outlive the reference it holds in its `top` field.

#### Lifetime Elision

*Lifetime Elision Rules* are a set of particular cases that the compiler will consider, and if your code fits these cases, you don't need to write the lifetimes explicitly.

Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*.

The compiler uses three rules to figure out what lifetimes references have when there aren't explicit annotations.

1.  Each parameter that is a reference gets its own lifetime parameter;
2.  If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters;
3.  If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because is a method, the lifetime of `self` is assigned to all output lifetime parameters.

With the three rules and without any lifetime annotations, the compiler is able to figure out the input lifetimes and the output lifetimes and if any of them can't be figured out, the code won't compile.

#### Lifetime Annotations in Method Definitions

When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters.

But we're not required to annotate the lifetime of the reference to `self` because of the first elision rule.

### The Static Lifetime

Many languages have the concept of "*static variables*" which means this kind of variables can live as long as the duration of the program, and Rust is not an exception.

In Rust, `static` is not a keyword, instead, it's a special lifetime annotation and it's used to decorate variables instead of references. Just like this:

```rust
let s: &'static str = "I hava a static lifetime";
```

### Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

