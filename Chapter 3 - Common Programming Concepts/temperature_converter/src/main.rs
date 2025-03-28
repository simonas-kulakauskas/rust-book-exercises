fn main() {
    let example_farenheit = 64.0;
    let example_celsius = 32.0;


    println!("{} Farenheit is equal to {} Celsius!", example_farenheit, convert_fahrenheit_to_celsius(example_farenheit));
    println!("{} Celsius is equal to {} Farenheit!", example_celsius, convert_celsius_to_farenheit(example_celsius));
}

fn convert_fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * (5.0/9.0)
}

fn convert_celsius_to_farenheit(c: f32) -> f32 {
    (c * (9.0/5.0)) + 32.0
}
