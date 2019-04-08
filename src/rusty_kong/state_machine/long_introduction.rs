use std::borrow::BorrowMut;
use super::GameState;

pub fn long_intro_enter(game_state: &mut GameState) {
    use super::super::video::TileMaps;

    info!("set bg1_cntl to long introduction tilemap.");
    game_state.set_bg(TileMaps::LongIntroduction);
//    let mut clone = game_state.system.clone();
//    let mut temp=clone.unwrap();
//    let temp1=temp.borrow_mut();
//    let mut video_gen = temp1.video_gen.unwrap().borrow_mut();
//    video_gen.set_bg(TileMaps::LongIntroduction);
}

pub fn long_intro_leave(game_state: &mut GameState) {}

pub fn long_intro_update(game_state: &mut GameState) {}