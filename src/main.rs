// enum Result<A , B>{
//     Ok(A),
//     Err(B)
// }

// fn main(){
//     let a1 = String::from("new");

//     new_printer{a2 : &a1};

//     println!("{}" , a1);
//     println!("{}" , a2);
// }

// fn new_printer(a2 : &String){
//     println!("{}" , a2);
// }

// fn main(){
//     let  mut vec =Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);
//     vec.push(6);


//    let vec2 =  even_filter(vec.clone());
//     println!("{:?}" , vec2 );
// }

// fn even_filter(vec :Vec<i32>) -> Vec<i32> {
//     let mut vec2 = Vec::new();

//     for val in vec {
//         if val%2 == 0 {
//             vec2.push(val);
//         }
//     }
//     return vec2;
// }

// use std::collections::HashMap;

// fn main(){
//     let mut users = HashMap::new();

//     users.insert(k::String::from("Wali") , v::20);
//     users.insert(k::String::from("Abdul") , v::1);

// let users_new = users.get("Wali");

//     match users_new {
//         Some(age) => println!("users age is {}" , age);
//         None => println!("No users found")
//     }
// }

use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("Wali"), 20);
    users.insert(String::from("Abdul"), 1);

    let users_new = users.get("Walis");

    match users_new {
        Some(age) => println!("User's age is {}", age),
        None => println!("No users found"),
    }
}