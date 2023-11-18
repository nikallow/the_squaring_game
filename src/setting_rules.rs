use crate::read_input::read_entered_number;

pub fn settings_rules() -> (u32, u32, u32) {
    println!("Welcome to the Squaring Game!");
    println!("You need to square the number by speed!\n");

    println!("Enter a minimal number to square: ");
    let min: u32 = read_entered_number();

    println!("Enter a maximum number to square: ");
    let max: u32 = read_entered_number();

    println!("Enter a number of questions: ");
    let num_of_quetions: u32 = read_entered_number();
    
    println!();
    (min, max, num_of_quetions)
}