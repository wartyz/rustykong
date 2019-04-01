use std::cell::RefCell;
use std::fmt::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::Sync;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum GameState {
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

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &GameState::None => write!(f, "state_nop"),
            &GameState::Boot => write!(f, "boot"),
            &GameState::Attract => write!(f, "attract"),
            &GameState::LongIntroduction => write!(f, "long_intro"),
            &GameState::HowHigh => write!(f, "how_high"),
            &GameState::GamePlay => write!(f, "game_play"),
            &GameState::PlayerDies => write!(f, "player_dies"),
            &GameState::PlayerWins => write!(f, "player_wins"),
            &GameState::KongRetreats => write!(f, "kong_retreats"),
        }
    }
}

struct StateHandlers {
    enter: fn(),
    update: fn(),
    leave: fn(),
    first_update: RefCell<bool>,
}

unsafe impl Sync for StateHandlers {}

struct States {
    previous: GameState,
    current: GameState,
    next: GameState,
}

static mut STATE: States = States {
    previous: GameState::None,
    current: GameState::None,
    next: GameState::None,
};

use self::attract::*;
use self::boot::*;
use self::how_high::*;
use self::long_introduction::*;
use self::state_nop::*;
use self::game_play::*;
use self::player_dies::*;
use self::player_wins::*;
use self::kong_retreats::*;
use std::cell::RefMut;
//use std::fmt::Display;
//use sdl2::Error;


mod attract;
mod boot;
mod how_high;
mod long_introduction;
mod state_nop;
mod game_play;
mod player_dies;
mod player_wins;
mod kong_retreats;

#[allow(dead_code)]
lazy_static! {
    static ref STATE_HANDLERS: Vec<StateHandlers>=vec!(
        StateHandlers{
            enter: state_nop,
            update:state_nop,
            leave: state_nop,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: boot_enter,
            update:boot_update,
            leave: boot_leave,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: attract_enter,
            update:attract_update,
            leave: attract_leave,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: long_intro_enter,
            update:long_intro_update,
            leave: long_intro_leave,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: how_high_enter,
            update:how_high_update,
            leave: how_high_leave,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: game_play_enter,
            update:game_play_update,
            leave: game_play_leave,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: player_dies_enter,
            update:player_dies_update,
            leave: player_dies_leave,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: player_wins_enter,
            update:player_wins_update,
            leave: player_wins_leave,
            first_update: RefCell::new(true),
            },
        StateHandlers{
            enter: kong_retreats_enter,
            update:kong_retreats_update,
            leave: kong_retreats_leave,
            first_update: RefCell::new(true),
            },
    );
}




pub fn game_state_go(state: GameState) {
    unsafe {
        STATE.next = state;
    }
}

pub fn game_state_init() {
    game_state_go(GameState::Boot);
}

fn get_state_handlers<'a>(state: GameState) -> &'a StateHandlers {
    &STATE_HANDLERS[state as usize]
}

pub fn game_state_update() {
    unsafe {
        if STATE.next != GameState::None {
            STATE.previous = STATE.current;
            let previous_handlers = get_state_handlers(STATE.previous);
            debug!("llamando {}_leave.", STATE.previous);
            (previous_handlers.leave)();
            let mut first_update = previous_handlers.first_update.borrow_mut();
            *first_update = true;

            STATE.current = STATE.next;
            debug!("transición a: {}.", STATE.current);
            STATE.next = GameState::None;

            let current_handlers = get_state_handlers(STATE.current);
            debug!("llamando {}_enter.", STATE.current);
            (current_handlers.enter)();
        } else {
            let handlers = get_state_handlers(STATE.current);
            let mut first_update = handlers.first_update.borrow_mut();
            if *first_update {
                debug!("llamando {}_update.", STATE.current);
                debug!("NOTA: solo la primera llamada");

                *first_update = false;
            }
            (handlers.update)();
        }
    }
}

