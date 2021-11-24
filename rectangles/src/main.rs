// An Example Program using Structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
        );

    // Refactoring with Tuples
    let rect1 = (20, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
        );

    // Refactoring with Structs
    let rect1 = Rectangle {
        width: 25,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
        );

    // Adding usefule functionality with Derived Traits
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    // This does not compile as Rectangle does not implement Display() trait
    // println!("{}", rectangle);
    println!("The rectangle is {:?}", rectangle);
    println!("The rectangle is {:#?}", rectangle);
    dbg!(20 * rectangle.width);
    dbg!(&rectangle);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
