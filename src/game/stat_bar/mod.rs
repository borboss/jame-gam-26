mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct StatBarPlugin;

impl Plugin for StatBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_render_bar)
            .add_system(update_bars.after(init_render_bar));
    }
}
