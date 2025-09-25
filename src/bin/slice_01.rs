//A string slice is a reference to a continuous part of a String. It's a "view" into that data without taking ownership of it.
//syntax: &s[start..end]
/*fn main(){
    let str = String::from("Hello world");

    let len = str.len();

    let slice = &str[..5]; //returns from starting index till i - 1; i.e from 0 to i-1

    let slice_second = &str[0..len]; //returns from 0 to whole length of the string

    let slice_third = &str[..]; //returns the whole string

    println!("{slice}, {slice_second}, {slice_third}");
}
*/

/*A slice is an immutable borrow. 
Rust's rules state that you cannot have a mutable borrow at the same time as an immutable borrow. */

// This function takes a string slice and returns the first word as a slice.
/*fn first_word(s: &str) -> &str {
    // Convert the string to an array of bytes.
    let bytes = s.as_bytes();

    // Iterate over the bytes with their index.
    for (i, &item) in bytes.iter().enumerate() {
        // If a space is found, return a slice from the start to the space.
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If no space is found, the whole string is one word.
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    // 'word' gets an immutable reference (a slice) to part of 's'.
    // This borrow must last until 'word' is last used.
    let word = first_word(&s);

    // ERROR: You try to get a mutable reference to 's' here to change it.
    // This is not allowed because the immutable reference in 'word' is still active.
    s.clear(); 

    // 'word' is used here, so its borrow is still active at the point of s.clear().
    println!("the first word is: {word}");
}*/

/*Of course. The error occurs because the main function is missing the first_word function it's trying to call.

Here is the complete program that produces the error, including the necessary first_word function.

Rust

// This function takes a string slice and returns the first word as a slice.
fn first_word(s: &str) -> &str {
    // Convert the string to an array of bytes.
    let bytes = s.as_bytes();

    // Iterate over the bytes with their index.
    for (i, &item) in bytes.iter().enumerate() {
        // If a space is found, return a slice from the start to the space.
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If no space is found, the whole string is one word.
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    // 'word' gets an immutable reference (a slice) to part of 's'.
    // This borrow must last until 'word' is last used.
    let word = first_word(&s);

    // ERROR: You try to get a mutable reference to 's' here to change it.
    // This is not allowed because the immutable reference in 'word' is still active.
    s.clear(); 

    // 'word' is used here, so its borrow is still active at the point of s.clear().
    println!("the first word is: {word}");
}
## Why It Fails
The code fails because of Rust's borrowing rules: you cannot have a mutable reference while an immutable reference exists.

let word = first_word(&s); creates an immutable reference (word) that points to the data inside s.

s.clear(); tries to create a mutable reference to change s.

println!(...{word}); uses the immutable reference word.

Because word is needed after s.clear() is called, its immutable borrow is still active. 
Rust forbids the mutable borrow from s.clear() to prevent a bug where word would be left pointing to empty, invalid data. */

/*## How to Fix It ✅
To fix this, you must ensure that the immutable borrow is finished before you try to create a mutable one. 
The easiest way is to use word and then modify's. */

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // Use the immutable borrow 'word' first.
    println!("the first word is: {word}");

    // The 'word' variable is no longer used after the println!,
    // so its immutable borrow ends here. Now it's safe to
    // take a mutable borrow.
    s.clear(); 

    println!("The string s is now: '{s}'");
}
//This version compiles and runs successfully because the scope of the immutable borrow (word) ends after the println!, allowing the mutable borrow (s.clear()) to proceed safely.