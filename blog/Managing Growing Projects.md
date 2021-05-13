## Managing Growing Projects

### Packages and Crates

A crate is a binary or library. The *crate root* is a source file that the Rust Compiler starts from and makes up the root module of you crate.

A *package* s one or more crates and provide a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates.

A package must contain at least one crate.

When we create a new project with Cargo, there is already a main.rs file under the src directory. Cause Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.

Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. **Cargo passes the crate root files to `rustc` to build the library or binary.**

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects.

#### Package Layout

Let's take a look at a typical package layout:

```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

-   `Cargo.toml` and `Cargo.lock` are stored in the root of your package;
-   Source code is stored in the `src` directory;
-   `src/lib.rs` is the default library file;
-   `src/main.rs` is the default executable file, other executable files can be stored in `src/bin` directory;

### Defining Modules to Control Scope and Privacy

*Modules* let us organize code within a crate into groups for readability and easy reuse. Modules also the privacy of items, which is whether an item can be used by outside code (*public*) or is an internal implementation detail and not available for outside use (*private*).

We define a module by starting with the `mod` keyword and then specify the name of the module and place curly brackets around the body of the module.

#### Make Module or its Contents Public with `pub`

A newly defined module and its contents are private by default, which means functions in other scopes are not allowed to access the module or its contents.

But if we pub a `pub` keyword in front of the module definition or the contents like functions inside the module.

What's the difference between this two?

-   Putting `pub` in front of the contents like functions or structs or modules makes them public and free to access in any other scopes. However, other contents without the `pub` keyword are still private.
-   Putting `pub` in front of the module makes it public, but the scopes that are allowed to access contents inside the module are constricted, which are its ancestor modules refer to it.

#### Starting Relative Paths with super

In java, we can use super to access the base class of the current class. In Rust, we can use super to refer to the parent module of the current module. It's just like the `..` syntax in a filesystem.

#### Structs Privacy Details 

Unlike module, elements of a struct are public by default, like we have mentions before, we can directly use dot notation to access fields. 

But if we put a struct inside a module, every element has to be made public before we access it unless the module is public.

And as said above, if we just make struct public, **all its elements will still be private** and only ancestor modules can access them.

Let's focus on enum, another type with similar usage, for one minute. In contrast, if we make an enum public in a private module. **All its variants are then public.**

### Bringing Paths into Scope with the`use` keyword

Although public functions are free to use, carrying a long path every time before using it is so exhausting and so redundant.

In C++, we can use the `using` keyword to bring a namespace into the current file. Similar solution exists in Rust.

Let's say, we have a demand of frequent access to functions inside a certain module. Then we can use the `use` keyword to bring the module into the current scope. 

If we just need to access one specific function or one specific variable inside a module, we can just bring the function or the variable inside the current scope too. And we can use it just like a locally defined function or variable. But this is recommended, cause it may be confused with local functions or variables, so it's not conformed to the Rust convention.

Note that **only public elements are allowed to bring into other scopes**, otherwise the whole privacy thing will be meaningless.

#### Providing New Names with the `as` keyword

In Python, if we need to import a module and hate its too long name, we are allowed to import the module in with a brand new name with the `as` keyword. It's really a convenient syntax characteristic. So Rust borrows this characteristic from Python.

Just like in Python, with the `use` keyword, we can import a module or other types with a new name.

#### Re-exporting Names with `pub use`

`pub use` make a name using in this file public to other files.

### Using External Packages

To use an external package, we first need to have the package. For standard packages, we can declare them in *Cargo.toml* and cargo will automatically download them if there isn't any.

Then we need to bring the package into the scope with `use` keyword. For instance:

```rust
use std::collections::HashMap;
```

Members of the Rust community have made many packages available at [crate.io](https://crates.io/).

If we need to use multiple items defined in the same crate or same module, here is a way to save some vertical space:

```rust
use std::{cmp::Ordering, io};
```

If we want to use multiple items inside a module and use module itself, we can literally use `self` to refer to the module, just as:

```rust
use std::io::{self, Write};
```

### Separating Modules into Different Files

I'll just use a very small project as an example, here is its file tree:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── front
    │   └── hosting.rs
    ├── front.rs
    ├── lib.rs
    └── main.rs

2 directories, 6 files
```

