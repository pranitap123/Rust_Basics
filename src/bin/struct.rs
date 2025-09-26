/*You’re trying to print the struct with {:#?}, which uses Rust’s Debug trait.
By default, structs don’t implement Debug, so you need to explicitly derive it. */
struct AlwaysEqual;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[derive(Debug)]
struct User{
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
}

fn main(){
//Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    let subject = AlwaysEqual; //Unit like struct

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let mut user1 = User{
        active : true,
        username : String::from("someuser1234"),
        email : String::from("someone@example.com"),
        sign_in_count : 1,
    };

    user1.email = String::from("anotheremail@email.com");

    println!("{:#?}", user1);

    fn build_user(email: String, username: String) -> User{
        User {
            active : true,
            username,
            email, 
            sign_in_count : 1,
        };

        let user2 = User {
            email : String::from("another@example.com"),
            ..user1
        };

    }
}
