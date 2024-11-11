use bevy::prelude::*;

pub const TEXT_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub fn text_bundle(text: impl Into<String>, asset_server: &Res<AssetServer>, style: TextStyle) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            color: TEXT_COLOR,
            font: asset_server.load("fonts/main-font.ttf"),
            ..style
        },
    )
}