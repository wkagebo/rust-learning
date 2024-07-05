fn convert_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / 1.8
}

fn main() {
    let fahrenheit = 80.0;
    let celsius = convert_to_celsius(fahrenheit);
    println!("{fahrenheit} degrees fahrenheit corresponds to {celsius} degrees celsius.");
}


