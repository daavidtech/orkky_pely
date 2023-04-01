use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::Window;


pub fn toggle_grab_cursor(window: &mut Window) {
	// let mut window = match windows.get_single_mut() {
	// 	Ok(w) => w,
	// 	Err(e) => {
	// 		warn!("Failed to get primary window for `initial_grab_cursor`! {}", e);
	// 		return;
	// 	}
	// };

	// window.cur

    // match window.cursor_grab_mode() {
    //     CursorGrabMode::None => {
    //         window.set_cursor_grab_mode(CursorGrabMode::Confined);
    //         // window.set_cursor_visibility(false);
    //     }
    //     _ => {
    //         window.set_cursor_grab_mode(CursorGrabMode::None);
    //         window.set_cursor_visibility(true);
    //     }
    // }
}

pub fn initial_grab_cursor(mut windows: Query<&Window>) {
	let mut window = match windows.get_single_mut() {
		Ok(w) => w,
		Err(e) => {
			warn!("Failed to get primary window for `initial_grab_cursor`! {}", e);
			return;
		}
	};

    // toggle_grab_cursor(&mut window);
}
