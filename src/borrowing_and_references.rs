// change in ownership
pub fn ownership(){
    let str = String::from("Tarun");
    let (size,str) = get_size(str);
    println!("{}",size);
    println!("{}",str);
}

fn get_size(str:String) -> (usize,String){
    return (str.len(),str);
}

//change in borrowing
pub fn borrowing(){
    let str = String::from("Tarun");
    let size = get_size_borrowing(&str);
    println!("{}",size);
    println!("{}",str);
}

fn get_size_borrowing(str:&String) -> usize{
    return str.len();
}

