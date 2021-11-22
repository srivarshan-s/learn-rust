const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    
    // Mutable & immutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    println!("The value of three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    // Shadow Variables
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in inner scope: {}", y);
    }
    println!("The value of y in outer scope: {}", y);
    let spaces = "    ";
    println!("There are spaces {} in this line", spaces);
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces)
}
