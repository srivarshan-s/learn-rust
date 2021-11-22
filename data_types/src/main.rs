use std::io;

fn main() {

    // Type annotation
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Integers
    let _integer: u32 = 4;
    let _integer: i32 = -4;

    // Floating Point
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Numeric Operations
    let _sum = 5 + 10;
    let _diff = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0
    let _floored = 2.0 / 3.0; // Results in 0.6666666
    let _remainder = 43 % 5;

    // Boolean
    let _t = true;
    let _f: bool = false;

    // Character type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_a, b, _c) = tup;
    println!("The second value in the tuple is: {}", b);
    let _one = tup.0;
    let _two = tup.1;
    let three = tup.2;
    println!("The third value in the tuple is: {}", three);

    // Array
    let _array = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", 
    "June", "July", "August", "September", "October", "November", 
    "December"];
    let _array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let array = [3; 6]; // value is [3, 3, 3, 3, 3, 3]
    let _first = array[0];
    let _second = array[1];

    // Invalid array element access
    println!("Please enter an array index:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");
    let element = array[index]; // Will panic if index > 6
    println!("The value of element at index {} is: {}",
             index, element);
}








