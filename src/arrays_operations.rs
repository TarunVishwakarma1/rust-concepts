pub fn add_array(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a + b).collect();
}

pub fn sub_array(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a - b).collect();
}

pub fn mul_array(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a * b).collect();
}

pub fn div_array(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a / b).collect();
}

pub fn mod_array(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a % b).collect();
}