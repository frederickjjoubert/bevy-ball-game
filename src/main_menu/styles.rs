use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub static MAIN_MENU_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    column_gap: Val::Px(8.0),
    row_gap: Val::Px(8.0),
    ..default()
});

pub static BUTTON_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    ..default()
});

pub static IMAGE_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    width: Val::Px(64.0),
    height: Val::Px(64.0),
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..default()
});

pub static TITLE_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(600.0),
    height: Val::Px(120.0),
    ..default()
});

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(50.0),
        TextColor(Color::WHITE),
    )
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(32.0),
        TextColor(Color::WHITE),
    )
}
