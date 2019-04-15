use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use std::boxed::Box;

//use sdl2::controller::GameController;

use super::level::Level;
use super::player::JumpMan;
use super::SystemInterfaces;
use super::TileMaps;

use self::attract::*;
use self::boot::*;
use self::game_play::*;
use self::how_high::*;
use self::kong_retreats::*;
use self::long_introduction::*;
use self::player_dies::*;
use self::player_wins::*;
use self::state_nop::*;
use self::commands::*;


mod boot;
mod attract;
mod long_introduction;
mod how_high;
mod state_nop;
mod game_play;
mod player_dies;
mod player_wins;
mod kong_retreats;
mod commands;

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

struct StateHandlers {
    state: GameStates,
    first_update: RefCell<bool>,
    enter: fn(&GameStateContext),
    leave: fn(&GameStateContext),
    update: fn(&GameStateContext),
}

impl StateHandlers {
    pub fn perfom_enter(&self, context: &GameStateContext) {
        debug!("llamando {}_enter.", self.state);
        (self.enter)(context);
    }

    pub fn perfom_leave(&self, context: &GameStateContext) {
        debug!("llamando {}_leave.", self.state);
        (self.leave)(context);
        let mut first_update = self.first_update.borrow_mut();
        *first_update = true;
    }

    pub fn perfom_update(&self, context: &GameStateContext) {
        let mut first_update = self.first_update.borrow_mut();
        if *first_update {
            debug!("llamando {}_update.", self.state);
            debug!("NOTA: solo la primera llamada");
            *first_update = false;
        }
        (self.update)(context);
    }
}

pub struct GameStateContext {
    commands: RefCell<Vec<Box<GameCommand>>>
}

impl GameStateContext {
    pub fn new() -> GameStateContext {
        GameStateContext { commands: RefCell::new(vec![]) }
    }

    pub fn set_bg(&self, tile_map: TileMaps) {
        let mut commands = self.commands.borrow_mut();
        commands.push(Box::new(SetBackgroundGameCommand::new(Some(tile_map))));
    }


    pub fn sprite(&self, number: u8, x: u16, y: u16, tile: u16, palette: u8, flags: u8) {
        let mut commands = self.commands.borrow_mut();
        commands.push(Box::new(SpriteGameCommand::new(number, x, y, tile, palette, flags)));
    }

    pub fn transition_to(&self, new_state: GameStates) {
        let mut commands = self.commands.borrow_mut();
        commands.push(Box::new(TransitionToGameCommand::new(new_state)));
    }

    pub fn process(&self, game_state: &mut GameState, system: &mut SystemInterfaces) {
        let mut commands = self.commands.borrow_mut();
        for command in commands.iter_mut() {
            command.execute(game_state, system);
        }
    }
}

pub struct GameState {
    level: Level,
    player: JumpMan,
    next: GameStates,
    current: GameStates,
    previous: GameStates,
    handlers: Vec<Rc<StateHandlers>>,
}

pub struct SetBackgroundGameCommand {
    tile_map: Option<TileMaps>,
}

impl SetBackgroundGameCommand {
    fn new(tile_map: Option<TileMaps>) -> SetBackgroundGameCommand {
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
                Rc::new(StateHandlers {
                    state: GameStates::None,
                    enter: state_nop_enter,
                    update: state_nop_update,
                    leave: state_nop_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::Boot,
                    enter: boot_enter,
                    update: boot_update,
                    leave: boot_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::Attract,
                    enter: attract_enter,
                    update: attract_update,
                    leave: attract_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::LongIntroduction,
                    enter: long_intro_enter,
                    update: long_intro_update,
                    leave: long_intro_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::HowHigh,
                    enter: how_high_enter,
                    update: how_high_update,
                    leave: how_high_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::GamePlay,
                    enter: game_play_enter,
                    update: game_play_update,
                    leave: game_play_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::PlayerDies,
                    enter: player_dies_enter,
                    update: player_dies_update,
                    leave: player_dies_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::PlayerWins,
                    enter: player_wins_enter,
                    update: player_wins_update,
                    leave: player_wins_leave,
                    first_update: RefCell::new(true),
                }),
                Rc::new(StateHandlers {
                    state: GameStates::KongRetreats,
                    enter: kong_retreats_enter,
                    update: kong_retreats_update,
                    leave: kong_retreats_leave,
                    first_update: RefCell::new(true),
                }),
            ],
        }
    }

//    fn system(&mut self, system: &'static SystemInterfaces) {
//        let mut clone = self.system.as_ref().unwrap().clone();
//        let mut sys = (*clone).borrow_mut();
//        *sys = system;
//    }

    fn is_pending_transition(&self) -> bool {
        self.next != GameStates::None
    }

    fn transition_states(&mut self, context: &GameStateContext) {
        debug!("transición a: {}.", self.next);
        self.previous = self.current;
        self.current = self.next;
        self.next = GameStates::None;
        self.leave_previous(context);
        self.enter_current(context);
    }


    pub fn transition_to(&mut self, state: GameStates) {
        self.next = state;
    }

    fn enter_current(&self, context: &GameStateContext) {
        let handlers = self.handlers[self.current as usize].clone();
        handlers.perfom_enter(context);
    }

    fn leave_previous(&self, context: &GameStateContext) {
        let handlers = self.handlers[self.previous as usize].clone();
        handlers.perfom_leave(context);
    }

    fn update_current(&self, context: &GameStateContext) {
        let handlers = self.handlers[self.current as usize].clone();
        handlers.perfom_update(context);
    }

    pub fn update(&mut self, context: &GameStateContext) {
        if self.is_pending_transition() {
            self.transition_states(context);
        } else {
            self.update_current(context);
        }
    }
}





