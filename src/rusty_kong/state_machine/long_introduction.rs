use sdl2::controller::GameController;
use super::GameState;

pub fn long_intro_enter(game_state: &GameState) {
    use super::super::video::TileMaps;
    //use super::super::video::video_set_bg;
    //video_set_bg(TileMaps::LongIntroduction);
}

pub fn long_intro_leave(game_state: &GameState) {}

pub fn long_intro_update(game_state: &GameState, controller: &GameController) {}