use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::*;
use crate::game::SimulationState;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    // mut commands: Commands,
    keyboart_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboart_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            // commands.insert_resource(NextState(Some(AppState::Game)));
            next_app_state.set(AppState::Game);
            println!("State Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    // mut commands: Commands,
    keyboart_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboart_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            // commands.insert_resource(NextState(Some(AppState::MainMenu)));
            // commands.insert_resource(NextState(Some(SimulationState::Paused)));
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("State MainMenu");
        }
    }
}

pub fn exit_game(
    keyboart_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboart_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("The final score is {}", event.score.to_string());
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}
