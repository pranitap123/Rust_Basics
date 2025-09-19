//commands to run files inside bin: $ cargo run --bin stack_heap
fn main(){
    stack_fn();
    heap_fn();
    update_string_fn();
}

fn stack_fn(){
    let a = 50;
    let b =50;
    let c = a + b;
    println!("The addition of {} and {} is {} ", a, b, c);
}

fn heap_fn(){
    let str_1 = String::from("Hello");
    let str_2 = String::from("World");
    let combined = format!("{} {}", str_1, str_2);
    println!("The combined string is: {}", combined);
}
//Here’s what format! does:

//format! is a macro (like println!).

//Instead of printing to the console, it creates a new String with the given formatting.

//"{} {}" is the template string, and s1, s2 are inserted in place of {}.

//It allocates a new heap-allocated String containing "Hello World".

fn update_string_fn(){
    let mut s = String::from("Intial string");
    println!("Before update: {} ", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    s.push_str("and some additional text");
    
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    println!("After update: {} ",s);
}