use std::io;

fn main() {
    let mut chosen_number = String::new();

    println!("Enter in the number of which you want the nth value...");

    io::stdin()
        .read_line(&mut chosen_number)
        .expect("Input couldn't be read!");

    let chosen_number: i32 = chosen_number.trim().parse().expect("Wasn't able to convert to integer.");

    let mut i = 0;

    let mut first_number = 0;
    let mut second_number = 1;

    println!("{first_number}");

    while i < (chosen_number - 1) {
        let new_number = first_number + second_number;
        first_number = second_number;
        println!("{second_number}");
        second_number = new_number;
        i = i + 1;
    }

    println!("\nThe {chosen_number}th value of the Fibonacci sequence is: {second_number}");
}
