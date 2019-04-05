use sdl2::controller::GameController;
use super::GameState;

pub fn boot_enter(game_state: &GameState) {}

pub fn boot_leave(game_state: &GameState) {}

pub fn boot_update(game_state: &GameState, controller: &GameController) {}