fn main() {
    // normal match boilerplate
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("the maximum is configured to be {max}"),
    //     _ => (),
    // }

    // if let match
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}