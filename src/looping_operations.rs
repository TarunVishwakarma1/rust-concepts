pub fn for_loop(a: &[i32]) -> i32 {
    let mut sum = 0;
    for i in a {
        sum += i;
    }
    return sum; 
}

pub fn while_loop(a: &[i32]) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < a.len() {
        sum += a[i]; 
        i += 1;
    }
    return sum;
}

pub fn do_while_loop(a: &[i32]) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    loop {
        sum += a[i];
        i += 1;
    }
}