
use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use crate::types::Fps;
use crate::types::GameEntity;
use crate::types::GameState;
use crate::types::LifeLeft;
use crate::types::LifeLost;
use crate::types::You;


#[derive(Default)]
pub struct GameUiPlugin;


impl Plugin for GameUiPlugin {
	fn build(&self, app: &mut App) {app   
        .add_plugin(FrameTimeDiagnosticsPlugin::default())     
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_fps_ui)
                .with_system(setup_health_ui)

                
        )

        .add_system_set(
            SystemSet::on_update(GameState::Game)
			.with_system(update_health)
            .with_system(fps_display_system)
        );
    

    }
}


fn fps_display_system(diagnostics: Res<Diagnostics>, mut query: Query<(&Fps, &mut Text)>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for (_, mut text) in query.iter_mut() {
                text.sections[0].value = format!("FPS: {:.0}", average);
            }
        }
    }
}


fn setup_fps_ui(mut commands: Commands, asset_server: Res<AssetServer>,) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    commands.spawn((TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "FPS: 0.00".to_string(),
                    style: TextStyle {
                        font: font,
                        font_size: 30.0,
                        color: Color::WHITE,
                    },               
                },           
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
				top: Val::Px(10.0),
				left: Val::Px(10.0),
				right: Val::Auto,
				bottom: Val::Auto
            },
            ..Default::default()
        },    
        ..Default::default()    
    }, Fps ));


}






fn setup_health_ui(mut commands: Commands) {

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(550.0), Val::Percent(15.0)),
            position: UiRect {	
                left: Val::Px(370.0),
                top: Val::Px(660.0),
                ..Default::default()
            },
            border: UiRect::all(Val::Px(20.0)),
            ..Default::default()
        },
        background_color: Color::rgb(1.0, 1.0, 1.0).into(),
        ..Default::default()      
    });
	

	commands.spawn((
		NodeBundle {
			style: Style {
				size: Size::new(Val::Px(500.0), Val::Px(20.0)),
				position_type: PositionType::Absolute,
				position: UiRect {
					
					left: Val::Px(400.0),
					bottom: Val::Px(20.0),
					..Default::default()
				},
				border: UiRect::all(Val::Px(20.0)),
				..Default::default()
			},
			background_color: Color::RED.into(),
			..Default::default()
		},
		LifeLost
	));


    commands.spawn((
		NodeBundle {
			style: Style {
				size: Size::new(Val::Px(0.0), Val::Px(20.0)),
				position_type: PositionType::Absolute,
				position: UiRect {
					
					left: Val::Px(400.0),
					bottom: Val::Px(20.0),
					..Default::default()
				},
				border: UiRect::all(Val::Px(20.0)),
				..Default::default()
			},
			background_color: Color::GREEN.into(),
			..Default::default()
		},
		LifeLeft
	));



}

fn update_health(
	mut query: Query<(&LifeLeft, &mut Style)>,
	game_entities: Query<(&GameEntity, &You)> 
) {
	let (_, mut style) = match query.iter_mut().next() {
		Some(x) => x,
		None => return,
	};
	let (game_entity, _) = match game_entities.iter().next() {
		Some(x) => x,
		None => return,
	};
    let max_pixelwidth = 500.0;
    let health = max_pixelwidth * (game_entity.curr_health / game_entity.max_health);
    style.size.width = Val::Px(health);

}




