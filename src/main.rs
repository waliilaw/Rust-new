enum Result<A , B>{
    Ok(A),
    Err(B)
}

fn main(){
    let a1 = String::from("new");

    new_printer{a2 : &a1};

    println!("{}" , a1);
    println!("{}" , a2);
}

fn new_printer(a2 : &String){
    println!("{}" , a2);
}