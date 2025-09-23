fn main(){
    let mut s = String::from("hello");

    let r1 = &s; // Person r1 gets permission to read.
    let r2 = &s; // Person r2 gets permission to read.
    
    println!("{r1} and {r2}");
    // This is the LAST time r1 and r2 are used.
    // Their turn is now over. They've walked away from the whiteboard.
    
    // Now that the room is empty, it's safe for someone to write.
    let r3 = &mut s; // Person r3 walks up to the board. This is fine!
    println!("{r3}");
}