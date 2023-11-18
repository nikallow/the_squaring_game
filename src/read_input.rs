use std::io;

pub fn read_entered_number() -> Option<u32> {
    let mut entered_number = String::new();

    io::stdin()
        .read_line(&mut entered_number)
        .expect("Failed to read line!");

    match entered_number.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,   
    }
}