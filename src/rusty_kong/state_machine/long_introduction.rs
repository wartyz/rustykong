use std::borrow::BorrowMut;
use super::GameStateContext;

pub fn long_intro_enter(context: &GameStateContext) {
    use super::super::video::TileMaps;

    info!("set bg1_cntl to long introduction tilemap.");
    context.set_bg(TileMaps::LongIntroduction);
}

pub fn long_intro_leave(context: &GameStateContext) {}

pub fn long_intro_update(context: &GameStateContext) {}