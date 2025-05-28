use bevy::prelude::*;
use bevy_pancam::{self, PanCamPlugin};
mod models;
mod systems;
pub mod utils;
use crate::systems::setup::setup;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .add_systems(Startup, setup);
    app.run();
}
