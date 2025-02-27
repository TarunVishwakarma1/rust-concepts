pub fn add_vector(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a + b).collect();
}

pub fn sub_vector(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a - b).collect();
}

pub fn mul_vector(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a * b).collect();
}

pub fn div_vector(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a / b).collect();
}

pub fn mod_vector(a: &[i32], b: &[i32]) -> Vec<i32> {
    return a.iter().zip(b.iter()).map(|(a, b)| a % b).collect();
}
