use std::time::Duration;

pub fn print_results(num_of_questions: &u32, num_of_right_questions: &u32, game_duration: &Duration) {
    println!("RESULTS:");
    println!("Total questions: {}", num_of_questions);
    println!("Number of right questions: {:?}", num_of_right_questions);
    println!("Time: {:?}", game_duration);
    println!("Medial time: {:?}", *game_duration / *num_of_questions);
}
