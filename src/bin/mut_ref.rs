fn main(){
    let mut s = String::from("hello");

    let r1 = &s; // Person r1 gets permission to read. //no problem
    let r2 = &s; // Person r2 gets permission to read. //no problem
    
    println!("{r1} and {r2}");
    // This is the LAST time r1 and r2 are used, it wont be used after this point.
    // Their turn is now over. They've walked away from the whiteboard.
    
    // Now that the room is empty, it's safe for someone to write.
    let r3 = &mut s; // Person r3 walks up to the board. This is fine!
    println!("{r3}");
}

/*fn main() {
    let mut s = String::from("hello");
    First, you create a String variable named s. The keyword mut is crucial; it marks s as mutable,
     meaning its value is allowed to be changed later. Without mut, you wouldn't be able to modify the string.

    change(&mut s);
    Next, you call the change function. Instead of giving it ownership of s, you lend it a mutable reference using &mut s. 
    This is like giving the function a key to access and modify the original s variable directly, rather than giving 
    it a copy.

    println!("{}", s); // This will print "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    The change function is designed to accept this "key." Its signature some_string: &mut String specifies that it 
    expects a mutable reference to a String. Inside the function, it uses this reference to call .push_str(), which 
    appends the text , world directly onto the original s variable that some_string points to.

} */