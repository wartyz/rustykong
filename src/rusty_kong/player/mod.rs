#[allow(dead_code)]
#[derive(Clone, Copy)]
struct JumpMan {
    x: u32,
    y: u32,
    lives: u32,
    score: u32,
}

static PLAYER1: JumpMan = JumpMan {
    y: 0,
    x: 0,
    lives: 3,
    score: 0,
};

pub fn player_draw() {}

pub fn player_update() {}