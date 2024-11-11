// Variables (nums,strings and locals)
// Create a function is_even

// println!("") <-- Macro

// "{}" <-- Dynamic Variable

// signed integer <-- negative and positive number both
// i32 <-- here 32 means bit

// fn is_even(num: i32) -> bool{
//     if num % 2 == 0 {
//         return true;
//     }
//     else{
//         return false;
//     }
// }

// Recursion
// fn fib(num : i32) -> i32{
//     if num == 1{
//         return 0;
//     }
//     else if num == 2{
//         return 1;
//     }
//     else{
//         return fib(num - 1) + fib( num - 2);
//     }
// }
// 0 1 1 2 3 5 8

// non - recursive
// fn fib(num: u32) -> u32 {
//     let mut num1 = 0;
//     let mut num2 = 1;
//     if num == 0 {
//         return num1;
//     }
//     if num == 1 {
//         return num2;
//     }
//     for _ in 1..num - 1 {
//         let temp = num2;
//         num2 = num2 + num1;
//         num1 = temp;
//     }
//     return num2;
// }

// fn get_str_length(str : String) -> usize{
//     str.chars().count()
// }

// struct User {
//     name : String,
//     email : String,
//     isactive : bool,
//     count_login : u64,
// }

// struct Rect{
//     length : u32,
//     width : u32,
// }

// impl Rect{
//     fn area(&self) -> u32{
//         self.length * self.width
//     }
// }

// Enums lets us enumerate over various types
enum Shape{
    Square(f64),
    Reactangle(f64,f64),
    Circle(f64),
}

fn calculatearea(shape : Shape) -> f64{
    //Pattern Matching 
    // Implicit return
    match shape{
        Shape::Reactangle(a,b)=>a*b,
        Shape::Circle(r)=>3.14*r*r,
        Shape::Square(a)=>a*a,
    }
}

fn main() {
    let circle = Shape::Circle(3.14);
    let square = Shape::Square(4.0);
    let rect = Shape::Reactangle(3.2, 4.3);
    println!("{}",calculatearea(circle));
    println!("{}",calculatearea(square));
    println!("{}",calculatearea(rect));
}
