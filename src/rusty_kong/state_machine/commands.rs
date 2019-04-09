use super::TileMaps;
use super::GameState;
use super::GameStates;
use super::SystemInterfaces;

pub trait GameCommand {
    fn execute(&self, game_state: &mut GameState, system: &mut SystemInterfaces);
}

// -----------------------------------------------------------------------------
//
// transition_to
//
// -----------------------------------------------------------------------------

pub struct TransitionToGameCommand {
    new_state: GameStates,
}

impl TransitionToGameCommand {
    pub fn new(new_state: GameStates) -> TransitionToGameCommand {
        TransitionToGameCommand { new_state }
    }
}

impl GameCommand for TransitionToGameCommand {
    fn execute(&self, game_state: &mut GameState, system: &mut SystemInterfaces) {
        game_state.transition_to(self.new_state);
    }
}

// -----------------------------------------------------------------------------
//
// set_bg
//
// -----------------------------------------------------------------------------

pub struct SetBackgroundGameCommand {
    tile_map: Option<TileMaps>,
}

impl SetBackgroundGameCommand {
    pub fn new(tile_map: Option<TileMaps>) -> SetBackgroundGameCommand {
        SetBackgroundGameCommand { tile_map }
    }
}

impl GameCommand for SetBackgroundGameCommand {
    fn execute(&self, game_state: &mut GameState, system: &mut SystemInterfaces) {
        //let clone = system.as_ref().unwrap().clone();
        //let sys = (*clone).borrow_mut();
        let clone = system.video_gen.as_ref().unwrap().clone();
        let tile_map = self.tile_map.unwrap();
        let mut video_gen = (*clone).borrow_mut();
        video_gen.set_bg(tile_map);
    }
}