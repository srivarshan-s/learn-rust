fn main() {

    // Creating a new Vector 
    let v: Vec<i32> = Vec::new();
    dbg!(v);
    let v = vec![1, 2, 3];
    dbg!(v);

    // Updating a Vector 
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    dbg!(v);

    // Reading elements of a Vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    // Reading invalid indices
    let v = vec![1, 2, 3, 4, 5];
    // Accessing like this causes the compiler to panic
    // let does_not_exist = &v[100];
    // dbg!(does_not_exist);
    // This gives a None value
    let does_not_exist = v.get(100);
    dbg!(does_not_exist);

    // Iterating over the values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        print!("{} ", i);
    }
    println!();
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
    dbg!(row);
}
