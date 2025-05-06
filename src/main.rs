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

fn main(){
    let  mut vec =Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);


   let vec2 =  even_filter(vec.clone());
    println!("{:?}" , vec2 );
}

fn even_filter(vec :Vec<i32>) -> Vec<i32> {
    let mut vec2 = Vec::new();

    for val in vec {
        if val%2 == 0 {
            vec2.push(val);
        }
    }
    return vec2;
}