fn main(){
    let x = 5;
    let y = x;
    //This is indeed what is happening, because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

    let str_1 = String::from("Hello");
    let str_2 = str_1;

    /*, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a 
    shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
     In this example, we would say that s1 was moved into s2. So, what actually happens is shown in Figure 4-4.
     
     That solves our problem! With only s2 valid, when it goes out of scope it alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. 
Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance. */
     
    let greet = String::from("Morning");
    let greet_dup = greet.clone();
    println!("{}, {}", greet, greet_dup);
    //If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
    //When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.

/*we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory
 for that variable. But Figure 4-2 shows both data pointers pointing to the same location. 
 This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously.
 Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities. */

 /*To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
  Therefore, Rust doesn’t need to free anything when s1 goes out of scope. 
 Check out what happens when you try to use s1 after s2 is created; it won’t work: */

 /*When you assign a new value to an existing variable, Rust immediately calls drop on the old value to free its memory. */

 let mut s = String::from("hello");
 s = String::from("ahoy");

 println!("{s}, world!");


 /*Here, the original "hello" string is dropped as soon as "ahoy" is assigned, since nothing refers to "hello" anymore. The variable s now points to the new heap data "ahoy". When printed, the output is "ahoy, world!". */
}