use std::convert::AsMut;
use std::rc::Rc;
use std::cell::RefCell;
use std::marker::Sync;
use std::fmt::Error;
use std::fmt::Display;
use std::fmt::Formatter;


use sdl2::controller::GameController;

use super::level::Level;
use super::player::JumpMan;
use super::SystemInterfaces;

use self::attract::*;
use self::boot::*;
use self::game_play::*;
use self::how_high::*;
use self::kong_retreats::*;
use self::long_introduction::*;
use self::player_dies::*;
use self::player_wins::*;
use self::state_nop::*;

mod boot;
mod attract;
mod long_introduction;
mod how_high;
mod state_nop;
mod game_play;
mod player_dies;
mod player_wins;
mod kong_retreats;


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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct States {
    previous: GameStates,
    current: GameStates,
    next: GameStates,
}

struct StateHandlers {
    state: GameStates,
    first_update: RefCell<bool>,
    enter: fn(&SystemInterfaces),
    leave: fn(&SystemInterfaces),
    update: fn(&SystemInterfaces),
}

impl StateHandlers {
    pub fn perfom_enter(&self, system: &SystemInterfaces) {
        debug!("llamando {}_enter.", self.state);
        (self.enter)(system);
    }

    pub fn perfom_leave(&self, system: &SystemInterfaces) {
        debug!("llamando {}_leave.", self.state);
        (self.leave)(system);
        let mut first_update = self.first_update.borrow_mut();
        *first_update = true;
    }

    pub fn perfom_update(
        &self,
        system: &SystemInterfaces) {
        let mut first_update = self.first_update.borrow_mut();
        if *first_update {
            debug!("llamando {}_update.", self.state);
            debug!("NOTA: solo la primera llamada");
            *first_update = false;
        }
        (self.update)(system);
    }
}

pub struct GameState {
    level: Level,
    player: JumpMan,
    current: GameStates,
    previous: GameStates,
    next: GameStates,
    //states: RefCell<States>,
    handlers: Vec<StateHandlers>,
}

impl GameState {
    pub fn init() -> GameState {
        let mut game_state = GameState::new();
        game_state.transition_to(GameStates::Boot);
        game_state
    }

    pub fn new() -> GameState {
        GameState {
            level: Level::new(),
            player: JumpMan::new(),
            previous: GameStates::None,
            current: GameStates::None,
            next: GameStates::None,
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

    fn is_pending_transition(&self) -> bool {
        self.next != GameStates::None
    }

    fn transition_states(&mut self, system: &SystemInterfaces) {
        debug!("transición a: {}.", self.next);
        self.previous = self.current;
        self.current = self.next;
        self.next = GameStates::None;
        self.leave_previous(system);
        self.enter_current(system)
    }

    fn next_handlers(&self) -> &StateHandlers {
        self.handlers(self.next)
    }

    fn current_handlers(&self) -> &StateHandlers {
        self.handlers(self.current)
    }

    fn previous_handlers(&self) -> &StateHandlers {
        self.handlers(self.previous)
    }

    fn handlers(&self, state: GameStates) -> &StateHandlers {
        &self.handlers[state as usize]
    }

    pub fn transition_to(&mut self, state: GameStates) {
        self.next = state;
    }

    fn enter_current(&self, system: &SystemInterfaces) {
        let current_handlers = self.current_handlers();
        current_handlers.perfom_enter(system);
    }

    fn leave_previous(&self, system: &SystemInterfaces) {
        let previous_handlers = self.previous_handlers();
        previous_handlers.perfom_leave(system);
    }

    fn perform_update(&self, system: &SystemInterfaces) {
        self.current_handlers().perfom_update(system);
    }

    pub fn update(&mut self, system: &SystemInterfaces) {
        if self.is_pending_transition() {
            self.transition_states(system);
        } else {
            self.perform_update(system);
        }
    }
}






