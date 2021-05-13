/**********************************************
  > File Name		: collections.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sun 24 Jan 2021 03:11:56 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

/*
 * Rust collections usage
 */

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

//string
fn st() {
    //create a new instance
    let s = String::new();
    //transfer a string into String type
    let words = "Hello World";
    let s = words.to_string();
    //or use from method
    let s = String::from("Hello World");

    //updating a String
    //append a str to the string with push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    //The push_str method don't take the ownership of the parameter

    //append a single character to the string with push
    s.push('s');

    //concatenation with the + operator and the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tok");
    let s = s1 + &s2;
    //or
    //let s = format!("{}{}", &s1, &s2);

    //slicing
    let ss = &s[0..2];
    println!("{}", ss);

    //slicing utf-8 encoded strings
    let uss = "你好世界";
    let suss = &uss[0..3];
    println!("{}", suss);
}

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

fn main() {
    //ve();
    st();
}
