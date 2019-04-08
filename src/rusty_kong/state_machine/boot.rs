use super::SystemInterfaces;
use super::GameStates;
use super::GameState;

pub fn boot_enter(game_state: &mut GameState) {
    game_state.transition_to(GameStates::LongIntroduction);
}

pub fn boot_leave(game_state: &mut GameState) {}

pub fn boot_update(game_state: &mut GameState) {}