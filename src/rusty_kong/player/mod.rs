#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct JumpMan {
    x: u32,
    y: u32,
    lives: u32,
    score: u32,
}

impl JumpMan {
    pub fn new() -> JumpMan {
        JumpMan {
            y: 0,
            x: 0,
            lives: 3,
            score: 0,
        }
    }
}
