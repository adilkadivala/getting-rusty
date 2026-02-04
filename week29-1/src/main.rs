// macros is a concept that allows you to define reusable code snippets that can be invoked with different inputs.
// They are particularly useful for reducing code duplication and enhancing code readability.

// difference between macro and function in rust // 1. Macros operate on the code itself, allowing for code generation and manipulation at compile time, while functions operate on values at runtime.
// 2. Macros can take a variable number of arguments and can work with different types without explicit type definitions, whereas functions have fixed parameter types and counts.
// use std::u32;
// use std::fmt::Display;

// declative macros
// macro_rules! say_hello {
//     () => {
//         println!("hey! hello")
//     };
// }

// #[derive(Debug)] // derive macros
// struct User {
//     name: String,
//     password: String,
//     age: u32,
// }

// procedural macros

// fn main() {
//     let u = User {
//         name: String::from("Alice"),
//         password: String::from("password123"),
//         age: 30,
//     };

//     print!("{:?}", u); // developer print for the debug trait
//     print!("{:#?}", u); // developer print with pretty output for the debug trait
//     print!("{}", u); // end user print for the display trait

//     say_hello!() // declarative macro invocation;
// }

// Write a macro that can take more than one function name as input and create functions for it.

/*

macro_rules! generate_functions {
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*
    };
}

generate_functions!(foo, bar, baz);

fn main() {
    foo();  // Prints: Hello from foo
    bar();  // Prints: Hello from bar
    baz();  // Prints: Hello from baz
}


*/

macro_rules! generate_functions {
    ($($func_name:ident),* ) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*

    };
}

fn main() {
    generate_functions!(foo, bar, baz);

    foo();
    bar();
    baz();
}
