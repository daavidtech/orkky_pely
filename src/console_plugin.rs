use bevy::prelude::*;
use bevy::reflect::Enum;

#[derive(Resource, Default)]
pub struct Console {
	pub active: bool,
	pub current_line: String,
	pub new_lines: Vec<String>
}

#[derive(Component)]
pub struct ConsoleUI;

#[derive(Component)]
pub struct ConsoleHistory;

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
	console_lines: Query<Entity, With<ConsoleUI>>,
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
								position: UiRect {
									right: Val::Percent(20.0),
									left: Val::Percent(20.0),
									bottom: Val::Percent(70.0),
									top: Val::Percent(0.0),
								},
								position_type: PositionType::Absolute,
								border: UiRect::all(Val::Px(20.0)),
								flex_direction: FlexDirection::Column,
								overflow: Overflow::Hidden,
								..Default::default()
							},
							background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
							..Default::default()
						},
						ConsoleUI,
					)).with_children(|parent| {
						parent.spawn((
							NodeBundle {
								style: Style {
									position: UiRect {
										right: Val::Percent(0.0),
										left: Val::Percent(0.0),
										bottom: Val::Percent(10.0),
										top: Val::Percent(0.0),
									},
									position_type: PositionType::Absolute,
									border: UiRect::all(Val::Px(20.0)),
									overflow: Overflow::Hidden,
									flex_direction: FlexDirection::ColumnReverse,
									..Default::default()
								},
								..Default::default()		
							},
							ConsoleHistory
						));

						parent.spawn(
							NodeBundle {
								style: Style {
									position: UiRect {
										right: Val::Percent(0.0),
										left: Val::Percent(0.0),
										bottom: Val::Percent(0.0),
										top: Val::Percent(90.0),
									},
									position_type: PositionType::Relative,
									border: UiRect::all(Val::Px(20.0)),
									overflow: Overflow::Hidden,
									..Default::default()
								},
								..Default::default()		
							}
						).with_children(|parent| {
							parent.spawn((
								TextBundle::from_section(
									"MISSÄ SÄ OOT??",
									TextStyle {
										font: asset_server.load("FiraSans-Bold.ttf"),
										font_size: 10.0,
										color: Color::WHITE
									},
								),
								ActiveConsoleLine,
							));
						});
						
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
	console_lines: Query<Entity, With<ConsoleHistory>>,
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
						NodeBundle {
							style: Style {
								size: Size {
									height: Val::Px(10.0),
									..Default::default()
								},
								..Default::default()
							},
							..Default::default()		
						}
					).with_children(|parent| {
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
				});
			},
			// TODO handle spaces and other similar keys
			_ => {
				console.current_line += key.variant_name();
			}
		};
	}
}
