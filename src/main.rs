mod gen_number;
mod read_input;
mod check_guess;
mod separator_output;
mod setting_rules;
mod print_results;

use separator_output::separator_output;
use gen_number::gen_number;
use read_input::read_entered_number;
use check_guess::check_square;
use setting_rules::settings_rules;
use print_results::print_results;

use std::time::Instant;


fn main() {
    let mut num_of_right_questions: u32 = 0;
    let (min, max, num_of_questions) = settings_rules();
    separator_output();

    // Start timer
    let start_time = Instant::now();

    for i in 1..=num_of_questions {
        let number: u32 = gen_number(min,max); // Generate random number
        println!("{}. Enter the square of the number {}: ", i, number);

        let entered_number = read_entered_number(); // Get user's number

        check_square(entered_number, number, &mut num_of_right_questions); // Check user's number
    }

    let game_duration = start_time.elapsed(); // End timer

    print_results(&num_of_questions, &num_of_right_questions, &game_duration)
}
