fn main() {
    // Infinite loop
    /*
    loop {
        println!("again!");
    }
    */

    // Nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter+=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // Conditional loops with while
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        print!("{} ", element);
    }
    println!("");

    // rev() prints it in reverse
    for number in(1..4).rev() {
        print!("{} ", number);
    }
    println!("LIFTOFF!");
}
