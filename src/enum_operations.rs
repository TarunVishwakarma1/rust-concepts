#[derive(Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

pub enum Shape{
    Circle(f32),
    Rectangle(f32, f32),
    Square(f32),
}

impl Shape{
    pub fn area(&self) -> f32{
        match self{
            Shape::Circle(radius) => radius * radius * 3.14,
            Shape::Rectangle(height, width) => height * width,
            Shape::Square(side) => side * side,
        }
    }
}  
