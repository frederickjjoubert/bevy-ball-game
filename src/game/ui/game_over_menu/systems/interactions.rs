use bevy::app::AppExit;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::Pressed;
use bevy::ui_widgets::{Activate, Button};

use crate::game::ui::game_over_menu::components::*;
use crate::game::ui::game_over_menu::styles::*;
use crate::AppState;

#[allow(clippy::type_complexity)]
pub fn update_button_colors(
    mut button_query: Query<
        (&Hovered, Has<Pressed>, &mut BackgroundColor),
        (
            With<Button>,
            Or<(With<RestartButton>, With<MainMenuButton>, With<QuitButton>)>,
        ),
    >,
) {
    for (hovered, pressed, mut color) in button_query.iter_mut() {
        *color = match (hovered.get(), pressed) {
            (_, true) => PRESSED_BUTTON.into(),
            (true, false) => HOVERED_BUTTON.into(),
            (false, false) => NORMAL_BUTTON.into(),
        };
    }
}

pub fn interact_with_restart_button(
    activate: On<Activate>,
    restart_button_query: Query<(), With<RestartButton>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if restart_button_query.contains(activate.entity) {
        app_state_next_state.set(AppState::Game);
    }
}

pub fn interact_with_main_menu_button(
    activate: On<Activate>,
    main_menu_button_query: Query<(), With<MainMenuButton>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if main_menu_button_query.contains(activate.entity) {
        app_state_next_state.set(AppState::MainMenu);
    }
}

pub fn interact_with_quit_button(
    activate: On<Activate>,
    quit_button_query: Query<(), With<QuitButton>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
) {
    if quit_button_query.contains(activate.entity) {
        app_exit_message_writer.write(AppExit::Success);
    }
}
