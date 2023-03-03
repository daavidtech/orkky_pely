use bevy::prelude::*;
use bevy::reflect::Enum;

#[derive(Resource, Default)]
pub struct Console {
	pub active: bool,
	pub current_line: String,
	pub new_lines: Vec<String>
}

#[derive(Component)]
pub struct ConsoleLines;

#[derive(Component)]
pub struct ActiveConsoleLine;

#[derive(Default)]
pub struct ConsolePlugin;


impl Plugin for ConsolePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(Console::default())
			.add_system(toggle_console)
			.add_system(console_keyboard_handler)
			.add_system(update_active_line);
	}
}

fn toggle_console(
	mut commands: Commands,
	mut console: ResMut<Console>,
	asset_server: Res<AssetServer>,
	keyboard: Res<Input<KeyCode>>,
	console_lines: Query<Entity, With<ConsoleLines>>,
) {
	let just_pressed = keyboard.get_just_pressed();

	for key in just_pressed {
		match key {
			KeyCode::F1 => {
				println!("F1 pressed");

				if console.active {
					let mut consolines = match console_lines.get_single() {
						Ok(entity) => commands.entity(entity),
						Err(_) => continue,
					};

					consolines.despawn();
					console.active = false;
				} else {
					commands.spawn((
						NodeBundle {
							style: Style {
								size: Size::new(Val::Px(800.0), Val::Percent(30.0)),
								position: UiRect {
									right: Val::Px(400.0),
									..Default::default()
								},
								border: UiRect::all(Val::Px(20.0)),
								flex_direction: FlexDirection::Column,
								..Default::default()
							},
							background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
							..Default::default()
						},
						ConsoleLines,
					)).with_children(|parent| {
						parent.spawn((
							TextBundle::from_section(
								"",
								TextStyle {
									font: asset_server.load("FiraSans-Bold.ttf"),
									font_size: 10.0,
									color: Color::WHITE,
								},
							),
							ActiveConsoleLine,
						));
					});
					console.active = true;
				}
			},
			_ => {}
		};
	}
}

fn update_active_line(
	console: ResMut<Console>,
	mut active_consoleline: Query<&mut Text, With<ActiveConsoleLine>>,
) {
	if !console.active {
		return;
	}

	let mut active_consoleline = match active_consoleline.get_single_mut() {
		Ok(entity) => entity,
		Err(_) => return,
	};

	active_consoleline.sections[0].value = console.current_line.clone();
}

fn console_keyboard_handler(
	mut commands: Commands,
	mut console: ResMut<Console>,
	asset_server: Res<AssetServer>,
	keyboard: Res<Input<KeyCode>>,
	console_lines: Query<Entity, With<ConsoleLines>>,
) {
	if !console.active {
		return;
	}

	let just_pressed = keyboard.get_just_pressed();

	for key in just_pressed {
		match key {
			KeyCode::Back => {
				console.current_line.pop();
			},
			KeyCode::Return => {
				let current_line = console.current_line.clone();
				console.new_lines.push(current_line.clone());
				console.current_line.clear();

				let mut console_lines = match console_lines.get_single() {
					Ok(entity) => commands.entity(entity),
					Err(_) => continue,
				};

				console_lines.with_children(|parent| {
					parent.spawn(
						TextBundle::from_section(
							current_line,
							TextStyle {
								font: asset_server.load("FiraSans-Bold.ttf"),
								font_size: 10.0,
								color: Color::WHITE,
							},
						)
					);
				});
			},
			// TODO handle spaces and other similar keys
			_ => {
				console.current_line += key.variant_name();
			}
		};
	}
}
