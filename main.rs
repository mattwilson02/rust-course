// fn main() {
//     println!("Hello, world!");
// }

// fn main() {
//     let x: i32 = 5;
//     let y: i32 = 10;

//     assert_eq!(x + y, 15);

//     println!("Success")
// }

// fn main() {
//     let mut x = 1;
//     x += 2;

//     assert_eq!(x, 3);
//     println!("Success");
// }

// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 15;

//     {
//         println!("The value of y is: {} the value of x is: {}", x, y);
//     }

//     println!("The value of y is: {} the value of x is: {}", x, y);
// }

// fn main() {
//     define_x();
// }

// fn define_x() {
//     let string: &str = "Hello";

//     println!("{}, world", string);
// }

// #[allow(unused_variables)]

// fn main() {
//     let x = 1;
// }

// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
//     println!("Success");
// }

// Destructuring

// fn main() {
//     let (x, y);

//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];

//     assert_eq!([x, y], [3, 2]);

//     println!("Success");
// }
