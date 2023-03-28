use bevy::prelude::{Res, DetectChanges};

use crate::game::player::components::HP;

pub fn game_over_handler(hp: Res<HP>) {
    if hp.is_changed() {
        if hp.hp <= 0 {
            println!("Game Over! ! ! ! !");
        }
    }
}