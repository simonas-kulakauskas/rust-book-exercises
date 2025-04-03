fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }


    // Option Pattern Matching
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matches are exhaustive, must cover all possible values!!! E.g. with these two options for 'catch-all'
    let dice_roll = 9;

    match dice_roll {
        3 => println!("Wow, number 3!!"),
        7 => println!("Lucky number 7!!"),
        other => println!("Boring..."), // Will pass through the value of itself when using other.
    }
    
    let dice_roll2 = 6;

    match dice_roll2 {
        3 => println!("Wow, number 3!!"),
        7 => println!("Lucky number 7!!"),
        _ => println!("Boring..."), // Won't pass through another value, just run what's on the left.
    }

    // TODO: Write about the 'if let' clauses
}
