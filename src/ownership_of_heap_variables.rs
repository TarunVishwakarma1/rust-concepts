pub fn chnage_in_immutable_int(){
    let x:i32 = 10;
    println!("{}",x);
}

pub fn chnage_in_mutable_int(){
    let mut x:i32 = 10;
    x = 20;
    println!("{}",x);
}

pub fn chnage_in_immutable_string(){
    let x:String = String::from("Hello");
    println!("{}",x);
    const Y:&str = "HELLO";
    println!("{}",Y);
}

pub fn chnage_in_mutable_string(){
    let mut x:String = String::from("Hello");
    x = String::from("World");
    println!("{}",x);
}