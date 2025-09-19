fn main(){
    let str_1 = String::from("Hello");
    let str_2 = str_1;
}
/*we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory
 for that variable. But Figure 4-2 shows both data pointers pointing to the same location. 
 This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously.
 Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities. */
 
 /*To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
  Therefore, Rust doesn’t need to free anything when s1 goes out of scope. 
 Check out what happens when you try to use s1 after s2 is created; it won’t work: */