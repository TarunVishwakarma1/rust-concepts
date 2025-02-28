// code duplication

// pub fn sum_u32(a:u32, b:u32) -> u32 {
//     a + b
// }

// pub fn sum_f32(a:f32, b:f32) -> f32 {
//     a + b
// }

//no code duplication using generics
pub fn sum<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}