pub fn add(a: f32, b: f32) -> f32 {
    return a + b
}

pub fn sub(a: f32, b: f32) -> f32 {
    return a - b
}

pub fn mul(a: f32, b: f32) -> f32 {
    return a * b
}

pub fn div(a: f32, b: f32) -> f32 {
    return a / b
}

pub fn modulus(a: i32, b: i32) -> i32 {
    return a % b
}

pub fn power(a: i32, b: i32) -> i32 {
    return a.pow(b as u32)
}

pub fn square(a: i32) -> i32 {
    return a * a
}

pub fn cube(a: i32) -> i32 {
    return a * a * a
}

pub fn square_root(a: i32) -> i32 {
    return a.isqrt()
}
