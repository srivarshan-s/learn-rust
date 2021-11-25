// The match control flow operator

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(&coin1));
    println!("{}", value_in_cents(&coin2));
    println!("{}", value_in_cents(&coin3));
    println!("{}", value_in_cents(&coin4));

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matches are exhaustive
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
