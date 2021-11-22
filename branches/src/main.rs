fn main() {
    // Code branching with for statement

    let number = 7;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    if number != 0 {
        println!("Something other than zero!");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number divisible by 4");
    } else if number % 3 == 0 {
        println!("Number divisible by 3");
    } else if number % 2 == 0 {
        println!("Number divisible by 2");
    } else {
        println!("Number is not divisible by 2, 3 or 4");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // This line shows error as both arms have different data types
    // let number = if condition { 5 } else { "six" };
    println!("{}", number);
}
