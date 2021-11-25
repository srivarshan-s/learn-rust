// Concise Control Flow with if let

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}",max),
        _ => (),
    }
}
