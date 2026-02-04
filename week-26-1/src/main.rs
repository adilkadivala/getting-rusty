// borrowing and references

// ownership rule:

/*  if variable var1 owns a value ex: "student", and then another variabel assigned var1

ex:
let var1 = String::from("new");
let var2 = var1;


so var1 is own the value, and it is owner, if var1 is removed than value will be also removed from the machine,

so var2 will point the null ::: dangling pointer issue

*/

/*

borrow ship

*/
/*

mutable reference :
*/
// fn main() {
//     let str = String::from("kadi");
//     let len = get_length(&str);
//     print!("{}", len);
// }

// fn get_length(str: &String) -> usize {
//     return str.len();
// }

// patter  mathcing

// enum Direction {
//     South,
//     West,
//     East,
//     North,
// }

// fn main() {
//     let dir = Direction::South;

//     match dir {
//         Direction::South => println!("go south"),
//         Direction::West => println!("go west"),
//         Direction::East => println!("go east"),
//         Direction::North => println!("go north"),
//     }
// }

// structs and impl

// use std::f32::consts::PI;

// struct Rectangle {
//     width: f32,
//     height: f32,
// }

// enum Shape {
//     Circle(f32),
//     Square(f32),
//     Rectangle(f32, f32),
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30.0,
//         height: 50.0,
//     };

//     let shape = Shape::Circle(10.0);
//     let shape_square = Shape::Square(10.0);
//     let shape_rect = Shape::Rectangle(10.0, 20.0);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
//     println!("The area of the circle is {} square pixels.", calculate_area(shape));
//     println!("The area of the square is {} square pixels.", calculate_area(shape_square));
//     println!("The area of the rectangle is {} square pixels.", calculate_area(shape_rect));
// }

// fn calculate_area(s: Shape) -> f32 {
//     match s {
//         Shape::Circle(r) => PI * r * r,
//         Shape::Square(side) => side * side,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }

// error handling with Result enum

use std::fs;

// enum Result <T, E> {
//     Ok(T),
//     Err(E),
// }


fn main() {
   let content = fs::read_to_string("a.txt");

    match content {
         Ok(data) => println!("File content: {}", data),
         Err(e) => println!("Error reading file: {}", e),
    }
}

