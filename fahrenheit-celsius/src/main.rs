fn fahrenheit2celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8000
}

fn celsius2fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8000) + 32.00
}

fn main() {
    println!("10 degrees fahrenheit is {} degrees celsius", fahrenheit2celsius(10.0));
    println!("10 degrees celsius is {} degrees fahrenheit", celsius2fahrenheit(10.0));
}
