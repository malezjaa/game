pub mod player;
pub mod menu;
mod screen;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::menu::{menu_plugin, setup_menu, GameState};

pub const TEXT_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .init_state::<GameState>()
        .add_systems(Startup, setup_menu)
        .add_plugins((menu_plugin))
        .run();
}