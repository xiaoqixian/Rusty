# Error Handling

Error handling requires you to acknowledge the possibility of an error and take some action before your code crashes. This requirement makes your program more robust.

Rust groups errors into two major categories: *recoverable* and *unrecoverable* errors.

For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Unlike most languages using Exceptions to handle errors, Rust has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

### Unrecoverable Errors with `panic!`

When the `panic!` macro executes, your program will print a failure message, unwind, clean up the stack and quit. It most commonly occurs when a bug of some kind has been detected and the programmer has no idea how to handle it.

*Unwinding* means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is *abort*, which means the program will quit and the operating system will clean up all the data. If you need to make your resulting binary file as small as possible, you can switch from unwinding to aborting upon a panic by adding `panic='abort'` to the appropriate `[profile]` sections in your *Cargo.toml* file.

For instance, switch from unwinding to aborting in your release file:

```
[profile.release]
panic = 'abort'
```

### Recoverable Errors with Result

Recoverable errors are usually not serious enough to stop the whole program, like the FILE NOT FIND error, we just need to tell the programmer that the file is not found, or the programmer can decide how to process the error in advance.

The `Result` enum can help you to handle potential errors, as follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` and `E` are both generic types, which we will talk about it later. `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.

Let's see how to use `Result` in file opening:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("problem opening the file: {:?}", error);
    }
}
```

#### Matching on Different Errors

When we can't open a file, there can be multiple reasons like: file not found, we don't have the permission to open the file, etc. For the first one reason, we may be able to create a new file, for the others, we still panic.

So here's the code:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error);
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    }
}
```

#### Shortcuts for Panic on Error: unwrap and expect

`unwrap` is a shortcut method that is implemented just like the `match` expression wrote above. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us.

```rust
let f = File::open("hello.txt").unwrap();
```

`expect` is similar to `unwrap`, it lets us to choose the `panic!` error message, so we can provide more detailed error messages.

```rust
let f = File::open("hello.txt").expect("Problem opening hello.txt");
```

#### Propagating Errors

When you're writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that if can decide what to do. This is known as *propagating errors* and gives more control to the calling code.

Define the function like this:

```rust
fn open_file(file_name: String) -> Result<String, io::Error> {
    let f = File::open(file_name);
    match f {
        Ok(file) => Ok(file),
        Err(error) => Err(error),
    }
}
```

Then we can handle the potential error in the calling code like this:

```rust
fn main() {
    let f = match open_file(String::from("hello.txt")) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening hello.txt"),
    }
}
```

#### A Shortcut for Propagating Errors: the `?` Operator

```rust
fn open_file(file_name: String) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}
```

The `?` is placed after the called function and before the semicolon, if the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the `Result` is an `Err`, the `Err` will be returned from the whole function as if we had the `return` keyword so the errors value gets propagated from the calling code.

There is a little detail that how the `?` operator works: error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, the error type received is converted into the error type defined in the return type of the current function. This useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the `from` function to define how to convert itself to the returned error type, the `?` operator takes care of the conversion automatically.

#### The `?` Operator Can Be Only Used in Functions That Return `Result`

As any expression that uses `?` operator might return a `Err` value, if a function doesn't return  such type value, then the `?` operator is not allowed in such functions.

### To `panic!` or Not to `panic!`

If someone calls your code and passes in values that don't make sense, the best choice might be to call `panic!` and alert the person using your library to the bug in their code so they can fix it during the development. Similarly, `panic!` is often appropriate if you're calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, if a failure is expected, it's more appropriate to return a `Result` than to make a `panic!` call.

When your code performs operations on values, your code should verify are valid first and panic if the values aren't valid. This is mostly for safety reasons: attempting to operate on invalid data can expose your code to vulnerabilities. Like when you're attempting to access the out-of-bounds memory, the standard library will panic.

ebebeb