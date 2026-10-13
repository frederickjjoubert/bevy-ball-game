use bevy::app::AppExit;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::Pressed;
use bevy::ui_widgets::{Activate, Button};

use crate::main_menu::components::*;
use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::AppState;

#[allow(clippy::type_complexity)]
pub fn update_button_colors(
    mut button_query: Query<
        (&Hovered, Has<Pressed>, &mut BackgroundColor),
        (With<Button>, Or<(With<PlayButton>, With<QuitButton>)>),
    >,
) {
    for (hovered, pressed, mut background_color) in button_query.iter_mut() {
        *background_color = match (hovered.get(), pressed) {
            (_, true) => PRESSED_BUTTON_COLOR.into(),
            (true, false) => HOVERED_BUTTON_COLOR.into(),
            (false, false) => NORMAL_BUTTON_COLOR.into(),
        };
    }
}

pub fn interact_with_play_button(
    activate: On<Activate>,
    play_button_query: Query<(), With<PlayButton>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if play_button_query.contains(activate.entity) {
        app_state_next_state.set(AppState::Game);
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
