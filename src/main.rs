use std::io;

fn main(){
    let a = [1,2,3,4,5];

    println!("Please enter an array index: ");

    //Creates a mutable String to store the user’s input.
    //Input from the console always comes as a string.

    let mut index = String::new();

    //Reads a full line from the console and stores it in index.
    //&mut index means the string can be modified.
    //expect("Failed to read line") will show an error if input fails.

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    //index.trim() → removes spaces and newline (\n) from user input.
    //.parse() → converts the string into a number.
    //usize → type for array indexing (unsigned integer).
    //If the input isn’t a valid number, the program crashes with the message "Index entered was not a number".

    let index: usize = index
             .trim()
             .parse()
             .expect("Index entered was not a number ");

    let element = a[index];

    println!("The value of the element at index {index} is: {element} ");
}
