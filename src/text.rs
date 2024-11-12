use bevy::prelude::*;

pub const TEXT_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub fn text_bundle(text: impl Into<String>, style: impl Bundle) -> impl Bundle {
    (Text::new(text), style)
}

pub fn text_style(size: f32, asset_server: &Res<AssetServer>) -> impl Bundle {
    (TextFont {
        font: asset_server.load("fonts/main-font.ttf"),
        font_size: size,
        ..default()
    }, TextColor(TEXT_COLOR))
}