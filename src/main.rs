pub mod player;
pub mod menu;
mod screen;
mod text;
mod cursor;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::menu::{menu_plugin, setup_camera, GameState};
use bevy::window::{CursorMoved, WindowTheme};
use bevy::winit::cursor::{CursorIcon, CustomCursor};
use crate::cursor::setup_cursor;

pub const GAME_TITLE: &str = "Chronos Game";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: GAME_TITLE.into(),
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }
        ))
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_cursor))
        .add_plugins(menu_plugin)
        .run();
}