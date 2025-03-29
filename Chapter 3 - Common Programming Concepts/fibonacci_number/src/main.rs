fn main() {
    let chosen_number = 15; // Nth value of fibonacci sequence, test value 4.
    let mut i = 0;

    let mut first_number = 0;
    let mut second_number = 1;

    while i < (chosen_number - 1) {
        let new_number = first_number + second_number;
        first_number = second_number;
        second_number = new_number;
        i = i + 1;
    }

    println!("{second_number}");
}
