fn main() {
    let s = "hello!";
    {
        let y = "HELLO!";
        println!("{}", s);
        println!("{}", y);
    }
    println!("{}", s);
    // Does not complie
    // println!("{}", y);

    let mut a = String::from("hello");
    a.push_str(", world!");
    println!("{}", a);

    // This works
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let s1 = String::from("STRING");
    let s2 = s1; // s1 is dropped ans s2 is given the value
                 // This does not work
                 // println!("{}", s1);
                 // This works
    println!("{}", s2);

    let s1 = String::from("STRING");
    let s2 = s1.clone(); // A copy is created
    println!("{} {}", s1, s2);

    // Ownership and Functions
    let s1 = String::from("hello");
    takes_ownership(s1);
    // Invalid as value is moved to function
    // println!("{}", s1);
    let x1 = 5;
    makes_copy(x1);
    // Valid as value is only copied to function
    println!("{}", x1);

    // Returning Values and Scope
    let s1 = gives_ownership();
    let s2 = String::from("hi");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s1);
    // Invalid as s2 is passed to takes_and_gives_back()
    // println!("{}", s2);
    println!("{}", s3);

    // To pass only value to function instead of moving it
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);
    println!("The length of {} is {}", s1, len);
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn makes_copy(num: i32) {
    println!("{}", num);
}

fn gives_ownership() -> String {
    let string = String::from("yours");
    string
}

fn takes_and_gives_back(string: String) -> String {
    string
}

fn calculate_length(string: String) -> (String, usize) {
    let length = string.len();
    (string, length)
}
