fn main() {

    // Creating a New String
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar");
    dbg!(s);
}
