use crate::*;

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOver;

impl Plugin for GameOverPlugin{
    fn build(&self, app: &mut App){
    app
    .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(clean_screen))
    .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(game_over));
    
}
} 

fn clean_screen(
all: Query<Entity>,
mut commands: Commands){
    for entity in &all{
        commands.entity(entity).despawn()

    }

}


fn game_over(mut commands: Commands,
mut asset_server: Res<AssetServer>){
    
    let icon = asset_server.load("gameover.png");

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
		ImageBundle {
			style: Style {
				
				margin: UiRect::all(Val::Auto),
				
				size: Size::new(Val::Px(1500.0), Val::Auto),
				..default()
			},
			image: UiImage(icon),
			..default()
		},
		GameOver,
	));

}


