enum Result<A , B>{
    Ok(A),
    Err(B)
}

fn main {
    let res = fs::read(path :"example.txt");

    match res {
        Ok(content : String ) => {
            println!("File contenmt : {}" , content);
        }
        Err(err : Error ) => {
            println!("Error found {}" , err);
        }
    }

}