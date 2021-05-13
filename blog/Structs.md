## Structs

### Defining and Instantiating Structs

Rust allows us to use structs to organize a group of related data.

We use such syntax to define a struct:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To instantiate it:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

If we want the instance to be mutable:

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

**Notice:** in Rust, it is not allowed to mark only certain fields as mutable.

###### A Tricky method to initiate certain fields

In a function, when the field names are the same as the parameters. Then these fields can be initiated in this way:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

###### Creating Instances From Other Instances With Struct Update Syntax

Sometimes we need a copy of another instance with a few fields to change. Assign the fields one by one with the respective fields in another instance is tedious.

So we can achieve it in this way:

```rust
let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
```

###### Tuple Structs

Tuple Structs can be used to create structs with anonymous fields.

Like this:

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

Note that even though two tuple structs are defined in a exactly same way, they are still different types.

#### Methods

*Methods* are different from functions in that they're defined within the context of a struct (or an enum or a trait type) and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

###### Defining Methods

To define a method, we use keyword `impl` to indicate which struct it belongs to. 

Let's say there is a `struct Rectangle`, we want to define a method to calculate its area.

```rust
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 2,
        height: 3,
    };
    println!("Rectangle area: {}", rect.area());
}
```

Multiple methods can be defined in the `imple` curly brackets, and multiple `impl` blocks of one struct are allowed.

Note that every method defined must carry a `&self` parameter, no matter if you are gonna use it or not. If you define a "method" in the `impl` block without `self` parameter, then it will just be a function (We'll talk about this later).

Also, mutable reference is allowed here, just use `&mut self` as the parameter instead, but only when the struct is mutable.

###### Associated Functions

As we mentioned above, functions defined within `impl` block will not be one of  the struct's methods, which means we are not able to call them like `struct.method()`. This kind functions are called *associated functions* because they are associated with the struct. The `String::from` is a typical associated function.

To call an associated function, we use `::` syntax with the struct name.

Associated functions are kind of like the *static methods* in C++.

