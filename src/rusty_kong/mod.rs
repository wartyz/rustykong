use std::cell::RefCell;

use sdl2;
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use self::state_machine::GameState;
use self::state_machine::GameStates;
use self::video::TileMaps;
use self::video::VideoGenerator;

mod level;
mod player;
mod state_machine;
mod video;


pub struct SystemInterfaces {
    //transition_target: RefCell<GameStates>,
    context: Option<Sdl>,
    game_state: Option<RefCell<GameState>>,
    event_pump: Option<RefCell<EventPump>>,
    video_gen: Option<RefCell<VideoGenerator>>,
    controller: Option<RefCell<GameController>>,
}

impl SystemInterfaces {
    pub fn new() -> SystemInterfaces {
        let mut system = SystemInterfaces {
            context: None,
            event_pump: None,
            controller: None,
            video_gen: None,
            game_state: None,
        };
        system.init();
        system
    }

    pub fn init(&mut self) {
        self.game_state = Some(RefCell::new(GameState::init(self)));
        self.context = Some(sdl2::init().unwrap());
        self.event_pump = Some(RefCell::new(self.context.as_ref().unwrap().event_pump().unwrap()));
        self.video_gen = Some(RefCell::new(VideoGenerator::init(self.context.as_ref().unwrap())));
        self.controller_init();
    }

    pub fn event_pump(&mut self) {
        let mut event_pump = self.event_pump.as_ref().unwrap().borrow_mut();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
            {
                let mut game_state = self
                    .game_state
                    .as_ref()
                    .unwrap()
                    .borrow_mut();
                game_state.update();
            }
            {
                let mut video_gen = self
                    .video_gen
                    .as_ref()
                    .unwrap()
                    .borrow_mut();
                video_gen.update();
            }
        }
    }

    fn controller_init(&mut self) {
        let subsystem = self.context
            .as_ref()
            .unwrap()
            .game_controller()
            .unwrap();
        let available = match subsystem.num_joysticks() {
            Ok(n) => n,
            Err(_e) => 0
        };
        self.controller = None;
        for id in 0..available {
            if subsystem.is_game_controller(id) {
                info!("Attempting to open controller {}", id);

                match subsystem.open(id) {
                    Ok(c) => {
                        info!("Success: opened \"{}\"", c.name());
                        self.controller = Some(RefCell::new(c));
                        break;
                    }
                    Err(e) => {
                        error!("failed: {:?}", e);
                        panic!();
                    }
                }
            } else {
                warn!("{} is not a game controller", id);
            }
        }
    }
}

pub fn game_run() {
    let mut system_interfaces = SystemInterfaces::new();
    system_interfaces.event_pump();
}

