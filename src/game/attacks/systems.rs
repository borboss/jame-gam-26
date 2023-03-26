use bevy::{prelude::*};

use bevy_tweening::Lerp;

use super::components::*;

pub fn fader(
    mut commands: Commands,
    mut fader_query: Query<(Entity, &mut Sprite, &FadeSoon), With<FadeSoon>>,
    time: Res<Time>,
) {
    for (entity, mut sprite, fade_component) in fader_query.iter_mut() {
        let a = sprite.color.a();
        sprite
            .color
            .set_a(a.lerp(&0.0f32, &(time.delta_seconds() / fade_component.time as f32 )));
        if sprite.color.a() <= 1.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_projectile(
    mut commands: Commands,
    mut projectile_query: Query<
        (Entity, &mut Transform, &mut SpawnedProjectile, &mut Sprite),
        With<SpawnedProjectile>,
    >,
    time: Res<Time>,
) {
    let x_min: f32 = 0.0 + 16.0;
    let x_max: f32 = 960.0 - 16.0;
    let y_min: f32 = (10.0f32 * 5.0f32) + 16.0;
    let y_max: f32 = 540.0 - 100.0 - 16.0;
    for (entity, mut projectile_transform, mut spawned_projectile, mut sprite) in
        projectile_query.iter_mut()
    {
        let direction: Vec3 = Vec3::new(
            spawned_projectile.direction.x,
            spawned_projectile.direction.y,
            0.0f32,
        );
        projectile_transform.translation += direction * 600.0 * time.delta_seconds();

        let mut translation: Vec3 = projectile_transform.translation;

        // Bound x position
        if translation.x < x_min {
            translation.x = x_min;
            spawned_projectile.direction.x *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        } else if translation.x > x_max {
            translation.x = x_max;
            spawned_projectile.direction.x *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        }

        // Bound y position
        if translation.y < y_min {
            translation.y = y_min;
            spawned_projectile.direction.y *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        } else if translation.y > y_max {
            translation.y = y_max;
            spawned_projectile.direction.y *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        }

        if direction.x > 0.0 {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }

        projectile_transform.translation = translation;
        if spawned_projectile.total_bounces >= spawned_projectile.max_bounces {
            // todo: animate poof
            commands.entity(entity).despawn();
        }
    }
}
