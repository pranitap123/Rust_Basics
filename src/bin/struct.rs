/*You’re trying to print the struct with {:#?}, which uses Rust’s Debug trait.
By default, structs don’t implement Debug, so you need to explicitly derive it. */

#[derive(Debug)]
struct User{
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
}

fn main(){

    let mut user1 = User{
        active : true,
        username : String::from("someuser1234"),
        email : String::from("someone@example.com"),
        sign_in_count : 1,
    };

    user1.email = String::from("anotheremail@email.com");

    println!("{:#?}", user1);
}