use sdl2::controller::GameController;
use super::GameState;

pub fn state_nop_enter(_game_state: &GameState) {}

pub fn state_nop_leave(_game_state: &GameState) {}

pub fn state_nop_update(_game_state: &GameState, _controller: &GameController) {}