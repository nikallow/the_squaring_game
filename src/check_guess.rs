use crate::separator_output::separator_output;


pub fn check_square(entered_number: u32, number: u32, num_of_right_questions: &mut u32) {
    let squared_number:u32 = number*number;
    if entered_number == squared_number {
        println!("Right!");
        separator_output();
        *num_of_right_questions += 1;
    } else {
        println!("{} squared is {}", number, squared_number);
        separator_output();
    }
}