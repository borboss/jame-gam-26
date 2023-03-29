use bevy::prelude::Component;
use bevy::prelude::Resource;
use bevy::prelude::Vec3;
use bevy::time::Timer;
use bevy::time::TimerMode;

pub const MANA_GEN_TIME:f32 = 2.0;

#[derive(Component, Default)]
pub struct Player {}

#[derive(Resource, Default)]
pub struct HP {
    pub hp: i32,
    pub max_hp: i32,
}
#[derive(Resource, Default)]
pub struct MP {
    pub mp: i32,
    pub max_mp: i32,
}

#[derive(Resource, Default)]
pub struct PlayerPosition {
    pub position: Vec3,
}

#[derive(Resource)]
pub struct ManaGenTimer {
    pub timer: Timer,
}
impl Default for ManaGenTimer {
    fn default() -> ManaGenTimer {
        ManaGenTimer {
            timer: Timer::from_seconds(MANA_GEN_TIME, TimerMode::Repeating),
        }
    }
}
