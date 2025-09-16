//Rust uses snake case i.e small letters as variable names separated by underscore
/* -----------------Arrays------------------------------------- 
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
*/

/*-----------Functions--------------------------- */

//fn means new function
fn main(){
     measurement_arguments(5, 'h');
     //double quotes are used to represent string and single quote is used to represent character
}
// you must declare the type of each parameter. 
//This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more helpful error messages if it knows what types the function expects.


fn measurement_arguments(value: i32, unit_label: char){
    println!("The value of measurement value is: {value}{unit_label} ");
}

/*-------Statements and Expressions:---------------
   fn main(){
    let y = {
        let x = 3; //expression
        x + 1 //expression
        //if we add a semicolon after 1 it turns into a statement and it is no longer an expression
    }

    println!("The value of y is: {y}");
   }
*/

/*Function with return value:
//Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). 
//There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32.

fn plus_one(x:i32) -> i32{
   //There are no function calls, macros, or even let statements in the plus_one function—just the x+1 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32.
   //Running this code will print The value of x is: 6. But if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error:  
     x+1
}

fn main(){
    let x = plus_one(5);
    println!("The value of x is: {x}");
}


*/