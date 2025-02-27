// mod arthematic_operations;
// mod string_operations;
//mod vector_operations;
//mod arrays_operations;
//mod looping_operations;
//mod ownership_of_heap_variables;
//mod borrowing_and_references;
//mod struct_operations;
//use struct_operations::Rect;
// mod enum_operations;
// use enum_operations::Direction;
// use enum_operations::Shape;
fn main() {

}


// fn arthematic_operations() {
//     println!("{}", arthematic_operations::add(1.1, 2.1));
//     println!("{}", arthematic_operations::sub(1.1, 2.1));
//     println!("{}", arthematic_operations::mul(1.1, 2.1));
//     println!("{}", arthematic_operations::div(1.1, 2.1));
//     println!("{}", arthematic_operations::modulus(1, 2));
//     println!("{}", arthematic_operations::power(2, 3));
//     println!("{}", arthematic_operations::square(2));
//     println!("{}", arthematic_operations::cube(2));
//     println!("{}", arthematic_operations::square_root(4));
// }

// fn string_operations() {
//     println!("{}", string_operations::concat("Hello", "World"));
//     println!("{}", string_operations::length("Hello"));
//     println!("{}", string_operations::reverse("Hello"));
//     println!("{}", string_operations::replace("Hello", "World", "There"));
//     println!("{:?}", string_operations::split("Hello World", " "));
//}

// fn vector_operations() {
//     let v1 = vec![1, 2, 3];
//     let v2 = vec![4, 5, 6];
//     println!("{:#?}", vector_operations::add_vector(&v1, &v2));
//     println!("{:#?}", vector_operations::sub_vector(&v1, &v2));
//     println!("{:#?}", vector_operations::mul_vector(&v1, &v2));
//     println!("{:#?}", vector_operations::div_vector(&v1, &v2));
//     println!("{:#?}", vector_operations::mod_vector(&v1, &v2));
// }

// fn arrays_operations() {
//     let a = [1, 2, 3];
//     let b = [4, 5, 6];
//     println!("{:#?}", arrays_operations::add_array(&a, &b));
//     println!("{:#?}", arrays_operations::sub_array(&a, &b));
//     println!("{:#?}", arrays_operations::mul_array(&a, &b));
//     println!("{:#?}", arrays_operations::div_array(&a, &b));
//     println!("{:#?}", arrays_operations::mod_array(&a, &b));
// }

// fn ownership_of_heap_variables(){
//     ownership_of_heap_variables::chnage_in_immutable_int();
//     ownership_of_heap_variables::chnage_in_mutable_int();
//     ownership_of_heap_variables::chnage_in_immutable_string();
//     ownership_of_heap_variables::chnage_in_mutable_string();
// }

// fn borrowing_and_references(){
//     borrowing_and_references::ownership();
//     borrowing_and_references::borrowing();
// }

// fn struct_operations(){
//     let rect = Rect{height: 10.0, width: 20.0};
//     println!("{}", rect.height);
//     println!("{}", rect.width);
//     println!("{}", rect.area());
//     println!("{}", rect.perimeter());
// }

// fn enum_operations(){
//     let dir = Direction::Up;
//     match dir{
//         Direction::Up => println!("Up"),
//         Direction::Down => println!("Down"),
//         Direction::Left => println!("Left"),
//         Direction::Right => println!("Right"),
//     }

//     let circle = Shape::Circle(10.0);
//     let rectangle = Shape::Rectangle(10.0, 20.0);
//     let square = Shape::Square(10.0);
//     println!("{}", circle.area());
//     println!("{}", rectangle.area());
//     println!("{}", square.area());
//     println!("{}", circle.perimeter());
//     println!("{}", rectangle.perimeter());
//     println!("{}", square.perimeter());
//     enum_operations::read_file("example.txt");
//     let index = enum_operations::find_first_a("hello world");
//     match index{
//         Some(index) => println!("{}", index),
//         None => println!("Not found"),
//     }
// }
