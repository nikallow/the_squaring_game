mod gen_number;
mod read_input;
mod check_guess;
mod separator_output;

use separator_output::separator_output;
use gen_number::gen_number;
use read_input::read_entered_number;
use check_guess::check_square;
use std::time::Instant;


fn main() {
    let min: u32 = 10;
    let max: u32 = 20;
    println!("Square the number by speed! Range: {} - {}", min, max);
    println!("Enter the number of attempts:");

    separator_output();

    let start_time = Instant::now();

    for i in 1..=5 {
        let number: u32 = gen_number(min,max);
        println!("{}. Enter the square of the number {}: ",i , number);

        let entered_number: u32 = match read_entered_number() {
            Some(num) => num,
            None => {
                println!("Enter the number!");
                continue;
            }            
        };

        check_square(entered_number, number);
    }

    let quiz_duration = start_time.elapsed();


    println!("Time: {:?}", quiz_duration);
    println!("Medial time: {:?}", quiz_duration/5);
}