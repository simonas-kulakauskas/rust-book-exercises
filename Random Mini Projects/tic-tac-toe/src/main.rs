fn main() {
    let mut row1: [char;3] = ['1', '2', '3'];
    let mut row2: [char;3] = ['4', '5', '6'];
    let mut row3: [char;3] = ['7', '8', '9'];
    let mut current_turn = 'X'; // Either 'X' or 'O's turns.

    println!("Current player's turn = {}", current_turn);
    print_all_rows(row1, row2, row3);
}

fn check_if_cell_empty(cell: &char) -> bool {
    if cell == &' ' {
        return true;
    } else {
        return false;
    }
}

fn print_all_rows(row1: [char; 3], row2: [char; 3], row3: [char; 3]) {
    println!(
        "|{}|{}|{}|\n|{}|{}|{}|\n|{}|{}|{}|",
        row1[0], row1[1], row1[2], row2[0], row2[1], row2[2], row3[0], row3[1], row3[2]
    );
}
