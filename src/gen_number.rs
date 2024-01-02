use rand::Rng;

pub fn gen_number(min: u32, max:u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}
