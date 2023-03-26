use bevy::prelude::*;

use crate::game::inventory::card_components::ProjectileType;

use super::components::*;

pub fn projectile_handle(projectile_query: Query<&Transform, With<SpawnedProjectile>>) {
    for projectile in projectile_query.iter() {}
}

pub fn move_projectile(mut projectile_query: Query<&mut Transform, With<(SpawnedProjectile)>>) {
    let x_min: f32 = 0.0 + 16.0;
    let x_max: f32 = 960.0 - 16.0;
    let y_min: f32 = (10.0f32 * 5.0f32) + 16.0;
    let y_max: f32 = 540.0 - 100.0 - 16.0;
    for mut projectile in projectile_query.iter_mut() {
        let mut translation: Vec3 = projectile.translation;

        // Bound x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        projectile.translation = translation;
    }
}
