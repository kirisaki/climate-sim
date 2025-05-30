use bevy::prelude::*;
use bevy_pancam::{self, PanCamPlugin};
mod models;
mod systems;
pub mod utils;
use crate::systems::setup::setup;
use crate::systems::update::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (render_hex_cells, update_hex_colors).chain()); // Ensure execution order with chain
    app.run();
}
