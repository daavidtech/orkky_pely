use bevy::prelude::*;

use crate::despawn::despawn_screen;
use crate::types::GameState;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
	fn build(&self, app: &mut App) {
		// As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
		app
			.add_system(splash_setup.in_schedule(OnEnter(GameState::Splash)))
			.add_system(countdown.in_set(OnUpdate(GameState::Splash)))
			.add_systems((
				splash_exit,
				despawn_screen::<OnSplashScreen>
			).in_schedule(OnExit(GameState::Splash)));
	}
}

fn splash_exit(
	mut commands: Commands,
	camera_2d: Query<Entity, With<Camera2d>>,
) {
	for entity in &camera_2d {
		commands.entity(entity).despawn_recursive();
	}
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(
	mut commands: Commands, 
	asset_server: Res<AssetServer>
) {
	let icon = asset_server.load("icon.png");
	// Display the logo
	commands.spawn(Camera2dBundle::default());
	commands.spawn((
		ImageBundle {
			style: Style {
				// This will center the logo
				margin: UiRect::all(Val::Auto),
				// This will set the logo to be 200px wide, and auto adjust its height
				size: Size::new(Val::Px(2000.0), Val::Auto),
				..default()
			},
			image: UiImage {
				texture: icon,
				..default()
			},
			..default()
		},
		OnSplashScreen,
	));
	// Insert the timer as a resource
	commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
	mut game_state: ResMut<NextState<GameState>>,
	time: Res<Time>,
	mut timer: ResMut<SplashTimer>,
) {
	if timer.tick(time.delta()).finished() {
		game_state.set(GameState::Menu);
	}
}
