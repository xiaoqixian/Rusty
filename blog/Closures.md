# Closures

### Closures: Anonymous Functions that Can Capture Their Environment

Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.

Unlike functions, closures can capture values from the scope in which they're defined.

#### Creating an Abstraction of Behavior with Closures

To create a closure:

```rust
let closure = |num| {
    num + 1
};
```

We create a quite short closure whose parameter is `num` and return value is `num + 1`, and we store the closure in the variable `closure`.

Closures don't require you to annotate the types of the parameters or the return value like `fn` functions do. The compiler can infer the type of parameters when you use the closure for the first time, and you're only allowed to pass the same type parameters then.

#### Storing Closures Using Generic Parameters and the `Fn` Traits

If we have an expensive closure (which means it takes a lot of CPU time to run it), and the resulting value may not be needed in some of the branches. 

One option to solve this issue is to save the result of the expensive closure in a variable for reuse and use the variable in each place we need the result, instead of calling the closure again.

Here come to a new issue: how are we supposed to store a closure? Cause when we define a struct, we need to explicitly annotate the type of the fields. So what is the type of a closure?

All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`. We'll cover the differences between them later. We add types to the `Fn` trait bound to represent the types of the parameters and return values the closures must have to match this trait bound, like we've discussed in the Generics chapter.

```rust
struct Cacher<T> where T: Fn(u32) -> u32 {
    closure: T,
    value: Option<u32>
}
```

Implementing the `Cacher`:

```rust
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

From the implementation, we can easily see the limitation of `Cacher`, the value field is not allowed to change. Once we calculate it out, it will still be the same.

#### Capturing the Environment with Closures

In a function, we are not allowed to use any variable that is out of its scope.  Unless we pass the variable as a parameter in. 

Same thing **won't** happen in closures. When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. This use of memory is overhead that we don't want to pay in more common cases where we want to execute code that doesn't capture its environment. 

Closures can capture values from their environment in three ways:

-   `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure's environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. 

    The `Once` part of the name represents the fact that the closure can't take ownership of the same variables more than once, so the closure can be called only once.

-   `FnMut` can change the environment because it mutably borrows values.

-   `Fn` borrows values from the environment immutably.

When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment. All closures implement `FnOnce` because they can all be called at least once. Closures that don't move the captured variables also implement `FnMut`, and closures that don’t need mutable access to the captured variables also implement `Fn`. 

If you want to force the closure to take ownership of the values it uses in the environment, you can use the `move` keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it's owned by the new thread.

