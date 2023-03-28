use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct SpawnedProjectile {
    pub direction: Vec2,
    pub total_bounces: u8,
    pub max_bounces: u8,
    pub x_radius: f32,
    pub y_radius: f32,
    pub damage: i32,
}
#[derive(Component)]
pub struct DamageEnemy {}
#[derive(Component)]
pub struct DamagePlayer {}

#[derive(Component)]
pub struct DontManageZ;