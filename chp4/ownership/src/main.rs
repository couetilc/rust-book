#![allow(dead_code)]

fn main() {
    // after shallow copy of string reference, borrow error occurs when old value is used, as
    // Strings are allocated on the stack and implement Drop trait
    // let s1 = String::from("hello!");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // deep copy string
    // let s1 = String::from("hello!");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // data types allocated on the stack so values can be copied
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // functions take ownership of heap data, cannot use afterwards
    // let s = String::from("hello");
    // takes_ownership(s);
    // let x = 5;
    // makes_copy(x);
    // println!("take {}", s);
    // println!("copy {}", x);

    // // functions can give back ownership
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);

    // // pass a string by reference
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // // change something we've borrowed
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);

    // // cannot not have more than one mutable reference at a time (errors as
    // // cannot have multiple mutable borrows)
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // // mutable references in different scopes work
    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    // // You can have multiple immutable references in the same scope. You cannot
    // // borrow as mutable if already borrowed as immutable.
    // let mut s = String::from("hello");
    // let r1 = &s; // yes
    // let r2 = &s; // yes
    // let r3 = &mut s; // NOPE
    // println!("{}, {}, and {}", r1, r2, r3);
    
    // // scope only extends to the last time a variable is used
    // let mut s  = String::from("hello");
    // let r1 = &s; // yes
    // let r2 = &s; // yes
    // println!("{} and {}", r1, r2);
    // let r3 = &mut s; // yes
    // println!("{}", r3);

    // // dangling reference (throws an error that suggests the `static` keyword,
    // // which in rust is related to the "lifetimes" feature) Basically, it points
    // // to a String pointer for an address in memory that has been deallocated
    // // because the String was not borrowed and went out of scope.
    // let reference_to_nothing = dangle();
    // let reference_to_something = no_dangle(): // borrows the string by moving its pointer

    // // String slices
    // let s = String::from("hello world"); // String pointer with memory address,
    //                                      // length, and capacity stored
    // let hello = &s[0..5]; // String slice with memory address and length stored
    // let world = &s[6..11];
    // let hello2 = &s[..5]; // (equivalent to previous statements)
    // let world2 = &s[6..];

    // // error, because borrowing an immutable referene then using it after a
    // // mutable borrow occurs during the ".clear()" call.
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // println!("the first word is: {}", word);

    // // string literals are slices, s is type &str
    // let s = "Hello, world!";

    // as string literals are string slices, lets make our api for first_word
    // more flexible by have accept a slice parameter instead of a string pointer
    fn first_word(s: &str) -> &str {
        for (i, &item) in s.as_bytes().iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    first_word("hello world");
    first_word(&String::from("hello world")[..]);

    // // other slices
    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3]; // type "&[i32]"

    // TODO program for once this chapter is over that takes a corpus of text
    // and calculates statistics on word frequency
}

// write a function that takesa string and returns the first word it finds
// in that string (a string with no spaces must be entirely one word)
// "&str" is the type for a "string slice".
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // (i, &item) are a pattern desctructuring of the tuple returned by enumerate.
    // iter() returns each element in a collection, while enumerate takes each
    // element and maps it to a tuple with the index of, and a reference to, the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// returns a dangling reference, will not compile
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn change(some_string: &mut String) { // must declare as mutable in order to modify String
    some_string.push_str(", world");
}

// references as function parameters is called "borrowing"
fn calculate_length(s: &String) -> usize { // s is a reference to a String
                                           // (a pointer to a String pointer)
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
