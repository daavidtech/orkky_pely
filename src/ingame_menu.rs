use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::KeyCode;
use bevy::prelude::NodeBundle;

use crate::constants::TEXT_COLOR;
use crate::types::Menu;



#[derive(Component)]
pub struct InGameMenu;

#[derive(Default)]
pub struct GameMenuPlugin;




const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
	Play,
	Settings,
	Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
		(Changed<Interaction>, With<Button>),
	>,
) {
	for (interaction, mut color, selected) in &mut interaction_query {
		*color = match (*interaction, selected) {
			(Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
			(Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
			(Interaction::Hovered, None) => HOVERED_BUTTON.into(),
			(Interaction::None, None) => NORMAL_BUTTON.into(),
		}
	}
}


impl Plugin for GameMenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(Menu::default())
			.add_system(toggle_exitmenu)
		    .add_system(button_system)
		    .add_system(menu_action);
			
	}
}
fn toggle_exitmenu(
	mut commands: Commands,
	mut menu: ResMut<Menu>,
	asset_server: Res<AssetServer>,
	keyboard: Res<Input<KeyCode>>,
	gamemenu: Query<Entity, With<InGameMenu>>,
) {
	let just_pressed = keyboard.get_just_pressed();
	let font = asset_server.load("FiraSans-Bold.ttf");
	
	let button_style = Style {
		size: Size::new(Val::Px(250.0), Val::Px(65.0)),
		margin: UiRect::all(Val::Px(20.0)),
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		..default()
	};
	
	let button_icon_style = Style {
		size: Size::new(Val::Px(30.0), Val::Auto),
		
		position_type: PositionType::Absolute,
		
		position: UiRect {
			left: Val::Px(10.0),
			right: Val::Auto,
			top: Val::Auto,
			bottom: Val::Auto,
		},
		..default()
	};
	
	let button_text_style = TextStyle {
		font: font.clone(),
		font_size: 20.0,
		color: TEXT_COLOR,
	};
	




	

	for key in just_pressed {
		match key {
			KeyCode::Escape => {
				println!("Escape pressed");

				if menu.active {
					let mut gamemenu = match gamemenu.get_single() {
						Ok(entity) => commands.entity(entity),
						Err(_) => continue,
					};

					gamemenu.despawn_recursive();
					menu.active = false;
				} else { let button_style = button_style.clone();
					let  button_icon_style = button_icon_style.clone();
					let button_text_style = button_text_style.clone();
					commands.spawn((
						NodeBundle {
							style: Style {
								position: UiRect {
									right: Val::Percent(39.0),
									left: Val::Percent(35.0),
									bottom: Val::Percent(29.0),
									top: Val::Percent(20.0),
								},
								position_type: PositionType::Absolute,
								border: UiRect::all(Val::Px(20.0)),
								flex_direction: FlexDirection::Column,
								overflow: Overflow::Hidden,
								..Default::default()
							},
							background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
							..Default::default()
						},
						InGameMenu
					))
					
					.with_children(|parent| {
						
						
			
						
						parent
							.spawn((
								ButtonBundle {
									style: button_style.clone(),
									background_color: NORMAL_BUTTON.into(),
									..default()
								},
								MenuButtonAction::Play,
							))
							.with_children(|parent| {
								let icon = asset_server.load("right.png");
								parent.spawn(ImageBundle {
									style: button_icon_style.clone(),
									image: UiImage {
										texture: icon,
										..default()
									},
									..default()
								});
								parent.spawn(TextBundle::from_section(
									"Restart",
									button_text_style.clone(),
								));
							});
						parent
							.spawn((
								ButtonBundle {
									style: button_style.clone(),
									background_color: NORMAL_BUTTON.into(),
									..default()
								},
								MenuButtonAction::Settings,
							))
							.with_children(|parent| {
								let icon = asset_server.load("wrench.png");
								parent.spawn(ImageBundle {
									style: button_icon_style.clone(),
									image: UiImage {
										texture: icon,
										..default()
									},
									..default()
								});
								parent.spawn(TextBundle::from_section(
									"Settings",
									button_text_style.clone(),
								));
							});
						parent
							.spawn((
								ButtonBundle {
									style: button_style,
									background_color: NORMAL_BUTTON.into(),
									..default()
								},
								MenuButtonAction::Quit,
							))
							.with_children(|parent| {
								let icon = asset_server.load("exitRight.png");
								parent.spawn(ImageBundle {
									style: button_icon_style,
									image: UiImage {
										texture: icon,
										..default()
									},
									..default()
								});
								parent.spawn(TextBundle::from_section("Quit", button_text_style));
							});
					});
					menu.active = true
                }
            },
			_ => {}
        }
    }
	
}


fn menu_action(
	interaction_query: Query<
		(&Interaction, &MenuButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	mut app_exit_events: EventWriter<AppExit>,
	
	
) {
	for (interaction, menu_button_action) in &interaction_query {
		if *interaction == Interaction::Clicked {
			match menu_button_action {
				MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => app_exit_events.send(AppExit),
                MenuButtonAction::Settings => app_exit_events.send(AppExit),
			
			}
		}
	}
}
