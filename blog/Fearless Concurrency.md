# Fearless Concurrency

Handling concurrent programming safely and efficiently is another of Rust's major goals. 

For simplicity's sake, we'll refer to many of the problems as *concurrent* rather than being more precise by saying *concurrent and/or parallel*. You may wonder about differences between *concurrency* and *parallelism*.

>   **Concurrency** is when two or more tasks can start, run and complete in overlapping time periods. It's can be achieved on a single-core machine. Just let the operating system arranges the CPU time reasonably. In a CPU time, there can be only task running. 
>
>   **Parallelism** is when tasks literally run at the same time, even in a CPU time, there can be multiple tasks running in multiple cores, if your machine has a multi-core processor.
>
>   Reference: https://stackoverflow.com/questions/1050222/what-is-the-difference-between-concurrency-and-parallelism

### Using Threads to Run Code Simultaneously

This chapter won't cover detailed introduction and goodnesses to *threads*, so if you're not familiar with them, you should acquire these knowledges first. We'll just focus on the usage of threads in Rust and how to avoid data races.

Many programming languages provide their own special implementation of threads. Programming language-provided threads are known as *green* threads, and languages that use green threads will execute them in the context of a different number of operating system threads. For this reason, the green-threaded model is called the *M:N* model: there are *M* green threads per *N* operating system threads, where *M* and *N* are not necessarily the same number.

#### Creating a New Thread with `spawn`

We use the function `thread::spawn` function to create a new thread and pass it a closure containing the code we want to run in the new thread.

```rust
thread::spawn(|| {
    println!("Hello world from the spawned thread.");
    thread::sleep(Duaration::from_milli(1));
})
```

Note that with this function, the new thread will be stopped when the main thread ends.

#### Waiting for All Threads to Finish Using `join` Handles

The return type of the function `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that we can the `join` method on it. Once we call the method, the thread that calls the method have to wait until the thread that spawned from the function which returns the `JoinHandle`.

#### Using `move` Closures with Threads

The `move` closure is often used alongside `thread::spawn` because it allows you to use data from one thread in another thread.

We have mentioned that the `move` keyword allows you to take the ownership of values that the closure uses in its environment.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

### Using Message Passing to Transfer Data Between Threads

One increasingly popular approach to ensuring safe concurrency is *message passing*, where threads communicate by sending each other messages containing data. 

One major tool Rust has for accomplishing message-sending concurrency is the *channel*. A channel has a transmitter and a receiver, one part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be *closed* is either the transmitter or receiver half is dropped.

We create a new channel using the `mpsc::channel` function; `mpsc` stands for *multiple producer, single consumer*. The `mpsc::channel` function returns a tuple, the first element is the sending end and the second element is the receiving end. 

```rust
let (tx, rx) = mpsc::channel();
```

Let's take an example to see how a channel works.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });
}
```

The transmitting end has a `send` method that sends a value to the receiving end and return a `Result<T,E>` type, if the receiving end has already been dropped and there's no where to send a value, the operation will return an error. In this example, we're calling `unwrap` to panic in case of error. For a real application, we would handle the error more specifically, you can review the error handling chapter for strategies for proper error handling.

Let's see the receiving end:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("received: {}", received);
}
```

So we let the spawned thread in charge of sending and the main thread in charge of receiving. The receiving end has a `recv` method to receive values. 

The `recv` method is a blocking method, a blocking method, means when you call it, the thread on which the method get called will be blocked until the method returns. 

If you don't want to block the thread, you can use the method `try_recv`. The `try_recv` method immediately returns a `Result<T, E>`: an `Ok` value holding a message if one is available and an `Err` if there aren't any messages this time. 

We can also treat `rx` as an iterator and use it in a `for` loop as such.

```rust
for received in rx {
    println!("Got: {}", received);
}
```

The loop will also block the thread, and every time the receiving end receives a value, the code in the loop block will run.

#### Channels and Ownership Transference

Let's see how channels and ownership work together. 

For the above code, when we use the `send` method to send a value to another thread will take the ownership of the value. Nor we can pass a reference as a parameter, cause we cannot sure how long is the value gonna live and the Rust compiler has to make sure that the value lives longer than the its any reference. 

The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.

#### Creating Multiple Producers by Cloning the Transmitter

We mentioned that `mpsc` was an acronym for *multiple producer, single consumer*. We know that the `channel` method can only produce one transmitting end and one receiving end. If we want to have multiple producer, we can clone the transmitting end.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
}
```

### Shared-State Concurrency

Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time. Usually, we use mutexes to manage multiple accessing to shared memory. 

#### Using Mutexes to Allow Access to Data from One Thread at a Time

Mutex is an abbreviation for *mutual exclusion*, a mutex allows only one thread to access some data at any time. 

When a thread want to access a shared data protected by mutex, it has to first attempt to require the mutex lock which cannot be required successfully if any thread is using the data. In this way, we can make sure that only thread accessing the data at any time. 

#### The API for `Mutex<T>`

The Rust provides a `Mutex<T>` type for mutexes demand in the `std::sync` module. 

According to the source code, a `Mutex<T>` is a struct which is able to contain a value. We can pass the value in as a parameter of the `new` method.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
```

When we want to access the data in the mutex, we need to call the `lock` method which will block the current thread and return a `Result` type.

After unwrapping, we get a `MutexGuard` type result. A `MutexGuard` is an RAII  **(**Resource Acquisition Is Initialization, an Object-Oriented Programming concept to avoid memory leak) guard to allow scoped unlock of the lock. When the guard goes out of scope, the mutex will be unlocked. To access the value, we need to dereference the result.

The call to `lock` would fail if another thread holding the lock panicked. In that case, no one would ever be able to get the lock, so we've chosen to `unwrap` and have this thread panic if we're in that situation. 

#### Sharing a `Mutex<T>` Between Multiple Threads

Since a mutex may get accessed by multiple threads, we need to make sure it is accessible in multiple threads.

However, if we just create a mutex variable in the main thread like:

```rust
let m = Mutex::new(0);
```

We're not allowed to use it in multiple threads, cause the thread using it will take the ownership of the value. So only types that implement the `Copy` trait is allowed to bring into multiple threads.

#### Multiple Ownership with Multiple Threads

The multiple owners for a single value mechanism may save us. We've mentioned that in the "Smart Pointer" chapter, a value can be owned by multiple threads and the value will not be dropped until all references go out of the scope. 

However, if wrap a mutex with `Rc`, we still get errors:

```rust
let m = Rc::new(Mutex::new(0));
```

The error message says: "`Rc<Mutex\<T>>`  cannot be sent between threads safely". This brings an issue that we haven't talked about yet: what types can be used in concurrent situations? The answer is all types that implement the `Send` trait, we will talk about this in more detail later.

So this method won't work out either.

#### Atomic Reference Counting with `Arc<T>`

`Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations, `Arc` is an abbreviation for *atomically reference counted*. And the `Arc<T>` type has the same API. 

Sum all this up, we get the code:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### Extensible Concurrency with the `Sync` and `Send` Traits

#### Allowing Transference of Ownership Between Threads with `Send`

The `Send` marker trait indicates that ownership of the type implementing `Send` can be transfered between threads. Almost every Rust type implements `Send` except `Rc<T>`.

#### Allowing Access from Multiple Threads with `Sync`

The `sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. Similar to `Send`, primitive types are `Sync`. 

