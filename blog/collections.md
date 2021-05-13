# Collections

## Storing Lists of Values with Vectors

### Vector

The first collection type we'll look at is `Vec<T>`, as known as a *Vector*. Vector allows you to store more than one value in a single data structure that puts all the values next to each other in memory.

###### How to use a vector

Let me explain the usage with a simple but classic instance:

```rust
//vector
fn ve() {
    //create a new instance
    let v: Vec<i32> = Vec::new();
    //it's mote common to create a vector that has initial values
    //as the compiler can infer the type i32 from the initial values, 
    //so we don't need the Vec<i32> annotation.
    let mut v = vec![1,2,3];
    
    //push in some values
    v.push(4);
    v.push(5);

    //two ways to access elements in a vector with index
    let third = &v[2];
    match v.get(3) {
        Some(third) => println!("vector third element is {}", third),
        None => println!("there is no third element"),
    }
    /*
     * differences between [] and get
     * []: give us a i32 type element or &i32 type if we use &
     * get: give us a Option<&T> type element
     */
    
    //what if we access an element with an unexisted index
    //let sixth = &v[5]; //panic when run the program
    let six = v.get(5); //return None

    /*
     * iterating over the values in a vector with the for loop
     */
    for i in &v {
        println!("{}", i);
    }
    //use mutable reference
    for i in &mut v {
        *i += 1;//we have to use the dereference operator(*) to get to the value in i before we can use the += operator
    }

    //pop: removes the last element from a vector and returns it
    println!("last element of the vector: {}", v.pop().unwrap());//unwrap is a Option<T> function to get the T value.

    //drain: creates a draining iterator that removes the specified range in the vector and yields the removed items.
    let dr: Vec<_> = v.drain(1..).collect();//collect is a function of iterator, we'll cover it later.
    assert_eq!(v.len(), 1);
}
```

Little details:

-   If we access elements with indexes beyond the length range:
    -   If use [], a warning will be reported, and when you run the program, it panicked because of "index out of bounds".
    -   If use `get`, a None value will be returned.

## Storing UTF-8 Encoded Text with Strings

The `String` type, which is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

Many of the same operations available with `Vec<T>` are available with `String` as well.

Let's see how Rust stores strings in memory.

#### String Internal Representation

A `String` is wrapper over a `Vec<u8>`.

```rust
let hello = String::from("Hola");
```

So the memory size the `hello` takes is 4, which means Rust need 4 bytes to store the String, one byte for each letter.

However, String supports UTF-8 encode, which means if the characters inside the String is not in ASCII encode set. It will need 2 bytes to store a single character. **So String indexing is not allowed.**

#### String Slices

Although String indexing is not allowed, we can still create String slices. You can use `[]` with a range to create a String slice containing particular bytes:

```rust
let hello = "你好";
let s = &hello[..2];
```

So the slice will contain the first two bytes data of `hello`, which is "你" cause each character takes 2 bytes to store.

You may want to ask what happens if we create a slice with an odd number of bytes. I'm gonna tell you the program will panic.

#### Methods for Iterating Over Strings

If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the `chars` method. 

```rust
for c in "你好世界".chars() {
    println!("{}", c);
}
```

### Storing Keys with Associated Values in Hash Maps

#### Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

If references are insert into hash maps, the values won't be moved into hash maps, but the values that the references point to must live at least as long as the hash map is valid.

```rust
//HashMap
fn hm() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    
    //insert a pair
    //hash maps are homogeneous: all of keys must have the same type, all of values must have the same type.
    map.insert(String::from("Blue"), 50);
    map.insert(String::from("yellow"), 40);

    //another way of constructing a hash map
    let teams = vec![String::from("Blue"), String::from("yellow")];
    let scores = vec![50, 40];
    let mut map: HashMap<_,_> = teams.into_iter().zip(scores.into_iter()).collect();

    //access pairs in hash map
    let team = String::from("Blue");
    let score = map.get(&team);//get method only accept reference type parameter, returns Option<T> type
    match score {
        Some(val) => {
            println!("The score of {} is {}", team, val);
        },
        None => {},
    }

    //check if a particular key has a corresponding value
    //if not, insert a value
    map.entry(String::from("Red")).or_insert(60);
    //the return value of the entry method is enum called Entry that represents a value that
    //might or might not exist. 
    //the or_insert method on Entry is defined to return a mutable reference to the value for
    //the corresponding Entry key it the key exists, and if not, inserts the parameter as
    //the new value of this key.

    //updating a value based on the old value
    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;//count is a reference, need a asterisk to dereference.
    }
    println!("{:?}", text_map);
}
```

