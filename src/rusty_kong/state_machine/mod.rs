use std::convert::AsMut;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::marker::Sync;

use sdl2::controller::GameController;

use super::level::Level;
use super::player::JumpMan;

use self::attract::*;
use self::boot::*;
use self::game_play::*;
use self::how_high::*;
use self::kong_retreats::*;
use self::long_introduction::*;
use self::player_dies::*;
use self::player_wins::*;
use self::state_nop::*;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum GameStates {
    None = 0 as isize,
    Boot,
    Attract,
    LongIntroduction,
    HowHigh,
    GamePlay,
    PlayerDies,
    PlayerWins,
    KongRetreats,
}

impl Display for GameStates {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &GameStates::None => write!(f, "state_nop"),
            &GameStates::Boot => write!(f, "boot"),
            &GameStates::Attract => write!(f, "attract"),
            &GameStates::LongIntroduction => write!(f, "long_intro"),
            &GameStates::HowHigh => write!(f, "how_high"),
            &GameStates::GamePlay => write!(f, "game_play"),
            &GameStates::PlayerDies => write!(f, "player_dies"),
            &GameStates::PlayerWins => write!(f, "player_wins"),
            &GameStates::KongRetreats => write!(f, "kong_retreats"),
        }
    }
}

struct States {
    previous: GameStates,
    current: GameStates,
    next: GameStates,
}

struct StateHandlers {
    state: GameStates,
    enter: fn(&GameState),
    leave: fn(&GameState),
    first_update: RefCell<bool>,
    update: fn(&GameState, &GameController),
}

impl StateHandlers {
    pub fn perfom_enter(self: &Self, game_state: &GameState) {
        debug!("llamando enter.");
        (self.enter)(game_state);
    }

    pub fn perfom_leave(self: &Self, game_state: &GameState) {
        debug!("llamando leave.");
        (self.leave)(game_state);
        let mut first_update = self.first_update.borrow_mut();
        *first_update = true;
    }

    pub fn perfom_update(
        self: &Self,
        game_state: &GameState,
        controller: &GameController) {
        let mut first_update = self.first_update.borrow_mut();
        if *first_update {
            debug!("llamando update.");
            debug!("NOTA: solo la primera llamada");
            *first_update = false;
        }
        (self.update)(game_state, controller);
    }
}

pub struct GameState {
    level: Level,
    player: JumpMan,
    states: States,
    handlers: Vec<StateHandlers>,
}

//impl<T> AsMut<T> for GameState {
//    fn as_mut(&mut self) -> &mut T {
//        (*self).as_mut()
//    }
//}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            level: Level::new(),
            player: JumpMan::new(),
            states: States {
                previous: GameStates::None,
                current: GameStates::None,
                next: GameStates::None,
            },
            handlers: vec![
                StateHandlers {
                    state: GameStates::None,
                    enter: state_nop_enter,
                    update: state_nop_update,
                    leave: state_nop_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::Boot,
                    enter: boot_enter,
                    update: boot_update,
                    leave: boot_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::Attract,
                    enter: attract_enter,
                    update: attract_update,
                    leave: attract_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::LongIntroduction,
                    enter: long_intro_enter,
                    update: long_intro_update,
                    leave: long_intro_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::HowHigh,
                    enter: how_high_enter,
                    update: how_high_update,
                    leave: how_high_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::GamePlay,
                    enter: game_play_enter,
                    update: game_play_update,
                    leave: game_play_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::PlayerDies,
                    enter: player_dies_enter,
                    update: player_dies_update,
                    leave: player_dies_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::PlayerWins,
                    enter: player_wins_enter,
                    update: player_wins_update,
                    leave: player_wins_leave,
                    first_update: RefCell::new(true),
                },
                StateHandlers {
                    state: GameStates::KongRetreats,
                    enter: kong_retreats_enter,
                    update: kong_retreats_update,
                    leave: kong_retreats_leave,
                    first_update: RefCell::new(true),
                },
            ],
        }
    }

    fn is_pending_transition(self: &Self) -> bool {
        self.states.next != GameStates::None
    }

    fn transition_states(&mut self) {
        debug!("transición a: {}.", self.states.next);
        self.states.previous = self.states.current;
        self.states.current = self.states.next;
        self.states.next = GameStates::None;
        self.leave_previous();
        self.enter_current();
    }

    fn next_handlers(self: &Self) -> &StateHandlers {
        self.handlers(self.states.next)
    }

    fn current_handlers(self: &Self) -> &StateHandlers {
        self.handlers(self.states.current)
    }

    fn previous_handlers(self: &Self) -> &StateHandlers {
        self.handlers(self.states.previous)
    }

    fn handlers(self: &GameState, state: GameStates) -> &StateHandlers {
        &self.handlers[state as usize]
    }

    pub fn transition_to(self: &mut GameState, state: GameStates) {
        self.states.next = state;
    }

    fn enter_current(self: &Self) {
        let current_handlers = self.current_handlers();
        current_handlers.perfom_enter(self);
    }

    fn leave_previous(self: &Self) {
        let previous_handlers = self.previous_handlers();
        previous_handlers.perfom_leave(self);
    }

    fn perform_update(self: &Self, controller: &GameController) {
        self.current_handlers().perfom_update(self, controller);
    }

    pub fn update(
        self: &mut GameState,
        controller: &GameController) {
        if self.is_pending_transition() {
            self.transition_states();
        } else {
            self.perform_update(controller);
        }
    }
}

mod boot;
mod attract;
mod long_introduction;
mod how_high;
mod state_nop;
mod game_play;
mod player_dies;
mod player_wins;
mod kong_retreats;


pub fn game_state_init() -> GameState {
    let mut game_state = GameState::new();
    game_state.transition_to(GameStates::Boot);
    game_state
}



