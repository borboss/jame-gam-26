use bevy::prelude::Component;
use bevy::prelude::Resource;
use bevy::prelude::States;
use bevy::prelude::Vec3;

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
