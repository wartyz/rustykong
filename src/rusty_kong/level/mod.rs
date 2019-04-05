#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Level {
    number: u32,
}

impl Level {
    pub fn new() -> Level {
        Level { number: 1 }
    }
}