fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    let s1 = String::from("hello");
    // Does not work
    change(&s1);

    // Mutable references
    let mut s1 = String::from("hello");
    mut_change(&mut s1);
    println!("{}", s1);

    // Multiple mutable references
    let mut s1 = String::from("hello");
    let r1 = &mut s1;
    // This line will throw error as multiple mutable references are not valid
    // let r2 = &mut s1;
    // println!("{} {}", r1, r2);

    // Combining mutable and immutable references
    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    // This throws error
    // let r3 = &mut s1;
    // println!("{} {} {}", r1, r2, r3);
    println!("{} {}", r1, r2);

    // Dangling references
    // Will throw error
    // let reference_to_nothin = dangle();
    // Solution is to return String directly
    let reference_to_something = no_dangle();
}

fn calculate_length(string: &String) -> usize {
    string.len()
}

// Does not work as string is only borrowed
fn change(string: &String) {
    // string.push_str(", world");
}

fn mut_change(string: &mut String) {
    string.push_str(", world");
}

// Will throw error as scope of s ends here leading to a dangling pointer
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
