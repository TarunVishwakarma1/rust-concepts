pub struct Rect{
    pub height: f32,
    pub width: f32,
}
impl Rect{
    pub fn area(&self) -> f32{
        self.height * self.width
    }

    pub fn perimeter(&self) -> f32{
        return 2.0 * (self.height + self.width);
    }
}
