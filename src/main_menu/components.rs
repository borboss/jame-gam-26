use bevy::{
    prelude::{Component, Deref, DerefMut},
    time::Timer,
};

#[derive(Component)]
pub struct MenuAnimationIndices {
    pub first: usize,
    pub last: usize,
    pub delete_on_end: bool,
    pub stop_on_end: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct MenuAnimationTimer(pub Timer);

#[derive(Component)]
pub struct MainMenuComponentMarker;
