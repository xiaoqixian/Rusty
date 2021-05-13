# Writing Automated Tests

### How to Write Tests

Tests are functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:

1.  Set up any needed data or state.
2.  Run the code you want to test.
3.  Assert the results are what you expect.

#### How to Write a Test Function

A test function is annotated with the `test` attribute, which means add `#[test]` on the line before `fn`. When you run your tests with the `cargo tst` command, Rust builds a test runner binary that runs the functions annotated with the `test` attribute and reports on whether each test function passes or fails.

When we run a test, the test can success or fail. **Tests fail when something in the test function panics.** 

Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

#### Testing Macros

The `assert!` macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to `true`.

The `assert_eq!` and `assert_ne!` are used to compare the result of the code under test to the value you expect the code to return, while the former is to make sure they are equal and the latter they are not.

#### Adding Custom Failure Messages

You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Custom messages are useful to document what an assertion means; when a test fails, you'll have a better idea of what the problem is with the code.

For example:

```rust
assert!(function(), "The function fails for {}", reason);
```

#### Checking for Panics with `should_panic`

When we add a `should_panic` attribute to our test function, it makes a test pass if the code inside the function panics; and a test fails if the code inside the function doesn't panic.

We place the `#[should_panic]` attribute after the `#[test]` attribute and before the test function it applies to.

However, code may panic for a reason we don't expect, which causes impression. To make `should_panic` tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute. 

For example:

```rust
#[test]
#[should_test(expected = "")]
```

#### Using `Result<T, E>` in Tests

```rust
#![allow(unused_variables)]
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

You can't use the `#[should_panic]` annotation on tests that use `Result<T,E>`. Instead, you should return an `Err` value directly when the test should fail.

### Controlling How Tests Are Run

You can specify command line options to change the default behavior of `cargo test`. 

Some command line options go to `cargo test`, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary. 

You can run `cargo test -- --help` to display the options you can use after the separator `--`.

#### Running Tests in Parallel or Consecutively

Tests run in parallel using threads by default. Make sure your tests don't depend on each other or on any shared state, including a shared environment. 

If you don't want to run the tests in parallel, you can use the command:

```rust
cargo test -- --test-threads=1
```

#### Showing Function Output

By default, if we call `println!` in a test function and the test passes, we won't see anything printed on the terminal. And if the test fails, we'll see whatever was printed to standard output with the rest of the failure message.

If we want to see the printed messages for passing tests as well, we can use the command:

```rust
cargo test -- --show-output
```

#### Running a Subset of Tests by Name

Sometimes, we don't need to run the whole set of tests. If you're working on a particular module of the project, you might just want to run the tests of that part. In this case, we can run a subset of tests by name with this command:

```
cargo test function_name
```

With the command, only tests with the certain name run.

But only running a function at a time is inefficient. So we can filter to run multiple tests at a time. We can specify part of a test name, and the tests match the name will be run. 

For example, if I run:

```
cargo test add
```

All tests whose name contain "add" run. 

#### Ignoring Some Tests Unless Specifically Requested

Sometimes, some tests are more important or time-consuming, so you may  want to run them separately. In this case, we can add a `#[ignore]` attribute to exclude this kind of tests so they won't run when we run `cargo test`.

If we want to run the ignored tests, run the command:

```
cargo test -- --ignored
```

### Test Organization

The Rust community thinks about tests in terms of two main categories: *unit tests* and *integration tests*. Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. Integration tests are entirely external to your library and use your code in the same any other external code would, using only the public interface and potentially exercising multiple modules per test.

#### Unit Tests

You'll put unit tests in the *src* directory in each file with the code they're testing. The convention is to create a module named `test` in each file to contain the test functions and to annotate the module with `cfg[test]`.

#### Integration Tests

In integration tests, they can only call functions that are part of your library's public API. Their purpose is to test whether many parts of your library work together correctly. 

We create a *tests* directory at the top level of our project directory, next to *src*. Cargo knows to look for integration test files in this directory. Each file in the *tests* directory in compiled as its own separate crate.

We don't need to annotate any code in *tests* directory with `#[cfg(test)]`. Cargo treats the `tests` directory specially and compiles files in this directory only when we run `cargo test`.

We can still run a particular integration test function by specifying the best function's name as an argument to `cargo test`, such as `cargo test --test integration_test`.

#### Submodules in Integration Tests

If we want to avoid having a certain file under the *tests* directory get tested, we should use a Rust naming convention. It is that the code in the *tests/common/mod.rs* file won't get tested.

#### Integration Tests for Binary Crates

If our project is a binary crate that only contains *src/main.rs* and doesn't have *src/lib.rs*, then we can't create integration tests for the project. Cause only library crates expose their functions so we can use in the integration tests. Binary crates are meant to be run on their own.