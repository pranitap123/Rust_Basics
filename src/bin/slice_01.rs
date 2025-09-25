fn main(){
    let str = String::from("Hello world");

    let len = str.len();

    let slice = &str[..5];

    let slice_second = &str[0..len];

    let slice_third = &str[..];

    println!("{slice}, {slice_second}, {slice_third}");
}