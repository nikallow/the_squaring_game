use crate::separator_output::separator_output;


pub fn check_square(entered_number: u32, number: u32, ) {
    let squared_number:u32 = number*number;
    if entered_number == squared_number {
        println!("Right!");
        separator_output()
    } else {
        println!("{} squared is {}", number, squared_number);
        separator_output()
    }
}