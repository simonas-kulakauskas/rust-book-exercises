use std::io;

// TODO: Call function to ask for temps, then use that to call the reresponding converison function and return back to original main loop.

fn main() {
    loop {
        let mut input = String::new();

        println!("Specify F to convert to Farenheit or C to convert to Celsius");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let input: char = input
        .trim()
        .to_lowercase()
        .parse()
        .expect("Invalid Input!");

        if input == 'f' {
            println!("You've chosen to convert to Farenheit!");
        } else if input == 'c' {
            println!("You've chosen to convert to Celsius!");
        } else {
            println!("Invalid character or nothing entered, try again!")
        }
    }
}

fn ask_for_temperature_to_convert(unit: char) -> f32 {
    let mut input = String::new();
    println!("What temperature would you like to convert?");
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        
    let input: f32 = input
    .trim()
    .parse()
    .expect("Invalid Input!");

    input
}

fn convert_fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

fn convert_celsius_to_farenheit(c: f32) -> f32 {
    (c * (9.0 / 5.0)) + 32.0
}
