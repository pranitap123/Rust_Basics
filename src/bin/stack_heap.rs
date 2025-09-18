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

fn update_string_fn(){
    let mut s = String::from("Intial string");
    println!("Before update: {} ", s);

    s.push_str("and some additional text");
    println!("After update: {} ",s);
}