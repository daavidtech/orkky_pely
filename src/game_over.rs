use bevy::prelude::*;

use crate::*;
use crate::despawn::despawn_screen;
use crate::types::GameState;

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOver;

impl Plugin for GameOverPlugin{
    fn build(&self, app: &mut App){
    	app
			.add_systems((
				clean_screen,
				game_over
			).in_schedule(OnEnter(GameState::GameOver)))
			.add_system(back_menu.in_set(OnUpdate(GameState::GameOver)))
			.add_systems((
				clean_screen,
				despawn_screen::<GameOver,>
			).in_schedule(OnExit(GameState::GameOver)));
	}
} 

fn clean_screen(
all: Query<Entity, Without<Window> >,
mut commands: Commands){
    for entity in &all{
        commands.entity(entity).despawn()
   
    }

}


fn game_over(mut commands: Commands,
asset_server: Res<AssetServer>){
    
    let icon = asset_server.load("gameover.png");

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
		ImageBundle {
			style: Style {
				
				margin: UiRect::all(Val::Auto),
				
				size: Size::new(Val::Px(1400.0), Val::Auto),
				..default()
			},
			image: UiImage{
				texture: icon,
				..default()
			},
			..default()
		},
		GameOver,
	));

}


fn back_menu(
	mut game_state: ResMut<NextState<GameState>>,
	keyboard: Res<Input<KeyCode>>
){
	if keyboard.just_pressed(KeyCode::Return){
		game_state.set(GameState::Menu);
	}
}


