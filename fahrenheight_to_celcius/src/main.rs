use std::io;

fn main() {
    println!("Fahrenheit to Celcius!");

    println!("Please enter a temperature in Fahrenheit:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    
    let temp: f64 = temp.trim().parse().expect("Expected number.");

    let result = to_celcius(temp);

    println!("You have converted {temp}F to {result}C!");
}

fn to_celcius(temp: f64) -> f64 {
    (temp - 32f64) * (5f64 / 9f64)
}
