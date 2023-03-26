use bevy::prelude::Component;

#[derive(Component)]
pub struct SpawnedProjectile {
    pub max_bounces: u8
}
#[derive(Component)]
pub struct DamageEnemy {}
#[derive(Component)]
pub struct DamagePlayer {}
#[derive(Component)]
pub struct FadeSoon {
    pub time: u8
}