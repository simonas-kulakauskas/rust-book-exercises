use std::io;

fn main() {
    loop {
        let choose_unit: char = ask_for_unit();
        let choose_temp: f32 = ask_for_temperature(&choose_unit);

        match choose_unit { // {:.1} -> This rounds the values to 1 decimal point.
            'f' => {
                println!(
                    "\n{:.1} degrees Celsius is equal to {:.1} Farenheit!\n",
                    choose_temp,
                    convert_celsius_to_farenheit(choose_temp)
                );
            }
            'c' => {
                println!(
                    "\n{:.1} degrees Farenheit is equal to {:.1} Celsius!\n",
                    choose_temp,
                    convert_fahrenheit_to_celsius(choose_temp)
                );
            }
            _ => {
                println!("Invalid Unit was passed!");
            }
        }
    }
}

fn ask_for_unit() -> char {
    loop {
        let mut choose_unit = String::new();
        println!("What unit would you like to convert to? ('f' for Farenheit, 'c' for Celsius)?");
        io::stdin()
            .read_line(&mut choose_unit)
            .expect("Unexpected unit input!");

        if choose_unit.trim().len() > 1 { // Accept only 1 character
            {
                println!("Too many characters, try again!");
            }
        } else if choose_unit.trim() != "" {
            let choose_unit: char = choose_unit
                .trim()
                .to_lowercase()
                .parse()
                .expect("Unable to convert to 'char' type.");

            if choose_unit == 'f' {
                return 'f';
            } else if choose_unit == 'c' {
                return 'c';
            } else {
                println!("Invalid unit, try again...");
            };
        } else {
            println!("Empty not accepted, try again!");
        };
    }
}

fn ask_for_temperature(unit: &char) -> f32 {
    loop {
        let mut choose_temperature = String::new();

        if unit == &'f' {
            println!("Input the value of Celsius you'd like converted to Farenheit.");
        } else if unit == &'c' {
            println!("Input the value of Farenheit you'd like converted to Celsius.");
        };

        io::stdin()
            .read_line(&mut choose_temperature)
            .expect("Unexpected input temperature!");

        if choose_temperature.trim() == "" {
            println!("Empty is not an accepted value! Try again...");
        } else if choose_temperature.trim().parse::<f32>().is_ok() { // check if input can be converted to a float, if it can, continue without killing program...
            let choose_temperature: f32 = choose_temperature
                .trim()
                .parse()
                .expect("Unable to convert 'f32' type.");
            return choose_temperature;
        } else {
            println!("Unexpected Value, Try again!.");
        };
    }
}

fn convert_fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

fn convert_celsius_to_farenheit(c: f32) -> f32 {
    (c * (9.0 / 5.0)) + 32.0
}