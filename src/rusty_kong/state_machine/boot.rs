use super::SystemInterfaces;
use super::GameStates;

pub fn boot_enter(system: &SystemInterfaces) {
    system.transition_to(GameStates::LongIntroduction);
}

pub fn boot_leave(system: &SystemInterfaces) {}

pub fn boot_update(system: &SystemInterfaces) {}