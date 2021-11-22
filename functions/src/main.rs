fn main() {
    another_function(-10);
    print_labeled_measurement(5, 'm');

    let _x = 5;
    let y = {
        let _x = 1;
        _x + 2
    };
    println!("{}", y); // Value is 3 not 7

    println!("{}", return_five());

    println!("{}", plus_three(7));
}

// Function
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// Function with parameter
fn print_labeled_measurement(x: u32, label: char) {
    println!("The measurement is: {}{}", x, label);
}

// Function with return value
fn return_five() -> i32 {
    5
}

// Function with return value and parameter
fn plus_three(num: i32) -> i32 {
    num + 3
}
