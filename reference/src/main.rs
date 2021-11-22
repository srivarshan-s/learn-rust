fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    let s1 = String::from("hello");
    // Does not work
    change(&s1);
}

fn calculate_length(string: &String) -> usize {
    string.len()
}

// Does not work as string is only borrowed
fn change(string: &String) {
    string.push_str(", world");
}
