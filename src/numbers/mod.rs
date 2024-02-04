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

// fn main() {
//     let (x, y);

//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];

//     assert_eq!([x, y], [3, 2]);

//     println!("Success");
// }

// fn main() {
//     let x: u8 = 127;

//     println!("{}", x)
// }

// fn main() {
//     let v: u16 = 38_u8 as u16;

//     print!("Success")
// }

// fn main() {
//     let x: i32 = 10;

//     assert_eq!("i32".to_string(), type_of(&x));
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);

//     print!("Success")
// }

pub fn numbers_fn() {
    let v1: u16 = 251_u16 + 8;
    let v2: i16 = i16::checked_add(251, 10).unwrap();

    println!("{}, {}", v1, v2)
}
