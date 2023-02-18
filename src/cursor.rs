use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::Window;


pub fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            // window.set_cursor_visibility(false);
        }
        _ => {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}

pub fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}
