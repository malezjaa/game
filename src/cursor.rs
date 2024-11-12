// use bevy::prelude::*;
// use bevy::window::SystemCursorIcon;
// use bevy::winit::cursor::{CursorIcon, CustomCursor};
// 
// #[derive(Resource)]
// pub struct CursorIcons(Vec<CursorIcon>);
// 
// pub fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.insert_resource(CursorIcons(vec![
//         CustomCursor::Image {
//             handle: asset_server.load("cursors/cursor_default.png"),
//             hotspot: (23, 24),
//         }
//             .into(),
//         CustomCursor::Image {
//             handle: asset_server.load("cursors/cursor_select.png"),
//             hotspot: (23, 24),
//         }
//             .into(),
//         CustomCursor::Image {
//             handle: asset_server.load("cursors/cursor_progress.gif"),
//             hotspot: (23, 24),
//         }
//             .into(),
//         SystemCursorIcon::Text.into(),
//     ]));
// }
// 
// pub fn handle_cursor(
//     mut commands: Commands,
//     window: Single<Entity, With<Window>>,
//     input: Res<ButtonInput<MouseButton>>,
//     cursor_icons: Res<CursorIcons>,
// ) {
//     if input.just_pressed(MouseButton::Left) {
//         commands
//             .entity(*window)
//             .insert(cursor_icons.0[1].clone());
//     } else if input.just_released(MouseButton::Left) {
//         commands
//             .entity(*window)
//             .insert(cursor_icons.0[0].clone());
//     }
// 
//     commands
//         .entity(*window)
//         .insert(cursor_icons.0[0].clone());
// }