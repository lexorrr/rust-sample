fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}")
}

fn five() -> i32 {
    5
}