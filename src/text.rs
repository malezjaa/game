use bevy::prelude::*;

pub const TEXT_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub fn text_bundle(text: impl Into<String>, asset_server: &Res<AssetServer>, style: impl Bundle) -> impl Bundle {
    (Text::new(
        text,
    ), style, TextFont {
        font: asset_server.load("fonts/Roboto-Regular.ttf"),
        ..default()
    })
}