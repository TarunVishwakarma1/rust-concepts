use std::fs;
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

enum Result{
    Ok(String),
    Err(String),
}

impl Shape{
    pub fn area(&self) -> f32{
        match self{
            Shape::Circle(radius) => radius * radius * 3.14,
            Shape::Rectangle(height, width) => height * width,
            Shape::Square(side) => side * side,
        }
    }
    pub fn perimeter(&self) -> f32{
        match self{
            Shape::Circle(radius) => 2.0 * radius * 3.14,
            Shape::Rectangle(height, width) => 2.0 * (height + width),
            Shape::Square(side) => 4.0 * side,
        }
    }
}  

pub fn read_file(path: &str){
    let content = fs::read_to_string(path);
    match content{
        Ok(content) => println!("{}", content),
        Err(e) => println!("{}", e),
    };
}