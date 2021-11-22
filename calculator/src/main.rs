use std::io;

fn read_f32() -> f32 {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Read failed!");
    let a: f32 = input.trim().parse::<f32>().expect("It is not a number!");
    return a;
}

fn read_string() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Read failed!");
    input = input.trim().parse::<String>().expect("Conversion failed!");
    return input;
}

fn main() {
    let mut a: f32 = read_f32();
    loop {
        let op: String = read_string();
        let b: f32 = read_f32();
        if op == "+" {
            a = a + b;
        }
        if op == "-" {
            a = a - b;
        }
        if op == "*" {
            a = a * b;
        }
        if op == "/" {
            a = a / b;
        }
        println!("{}", a);
    }
}
