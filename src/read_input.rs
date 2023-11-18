use std::io;

pub fn read_entered_number() -> u32 {
    loop {
        let mut entered_number = String::new();

        io::stdin()
            .read_line(&mut entered_number)
            .expect("Failed to read line!");

        match entered_number.trim().parse() {
            Ok(num) => {
                return num
            },
            Err(_) => {
                println!("Enter the number!");
                continue;
            },   
        }
    }
    
}