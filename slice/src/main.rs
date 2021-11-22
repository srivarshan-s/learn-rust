fn first_word(string: &String) -> &str {
    let bytes = string.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }
    &string[..]
}

fn first_word_str(string: &str) -> &str {
    let bytes = string.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }
    &string[..]
}

fn second_word(string: &String) -> &str {
    let bytes = string.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[i+1..];
        }
    }
    &string[..]
}

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);

    let mut s = String::from("hello world");
    let f_word = first_word(&s);
    let s_word = second_word(&s);
    println!("{}", f_word);
    println!("{}", s_word);
    // Throws error as refernce to s exist: f_word and s_word
    // s.clear();
    println!("{}", f_word);

    // Passing Slices as Parameters
    let my_string = String::from("hello world");
    let word = first_word_str(&my_string[0..6]);
    println!("{}", word);
    let word = first_word_str(&my_string[..]);
    println!("{}", word);
    let word = first_word_str(&my_string);
    println!("{}", word);
    let my_string_literal = "hello world";
    let word = first_word_str(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word_str(&my_string_literal[..]);
    println!("{}", word);
    let word = first_word_str(my_string_literal);
    println!("{}", word);
}
