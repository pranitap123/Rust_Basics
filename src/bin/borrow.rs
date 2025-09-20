fn main(){

    let str = String::from("Hello");
    //This creates a new String called s1 with the value "hello".
    //Unlike &str string slices, a String is owned and stored on the heap.
    let len = calculate_length(&str);
    //Instead of passing s1 directly, we pass &s1.
    //&s1 is a reference to s1.
    //This means:
    //Ownership of s1 is not moved into the function.
    //The function can borrow s1 and read from it without taking ownership.
    //If we didn’t use &s1 and instead passed s1, then s1 would move into the function, and we wouldn’t be able to use it later in println!.

    println!("The length of {str} is {len}.");

    //{s1} → prints the value of s1 → "hello".
    //{len} → prints the calculated length → 5.
}

fn calculate_length(s: &String) -> usize{
    //s: &String → This means the function borrows a reference to a String, it doesn’t own it.
    //s.len() → Calls the len method of String to get its length (number of characters).
    //The return type is usize (the standard integer type used for sizes and lengths in Rust).
    s.len()
}

/*Passing String directly → moves ownership, you can’t use it again in the caller.

Passing &String → borrows ownership, so you can still use it after the function call. */