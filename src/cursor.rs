use bevy::prelude::*;
use bevy::winit::cursor::CustomCursor;

pub fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CustomCursor::Image {
        handle: asset_server.load("cursor.png"),
        hotspot: (128, 128),
    }.into());
}