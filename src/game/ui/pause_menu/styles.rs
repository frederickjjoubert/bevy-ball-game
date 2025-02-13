use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub static PAUSE_MENU_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    position_type: PositionType::Absolute, // Needed to display separately from HUD.
    display: Display::Flex,                // Hidden by Default
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    ..default()
});

pub static PAUSE_MENU_CONTAINER_NODE: std::sync::LazyLock<Node> =
    std::sync::LazyLock::new(|| Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(400.0),
        height: Val::Px(400.0),
        column_gap: Val::Px(8.0),
        row_gap: Val::Px(8.0),
        ..default()
    });

pub static BUTTON_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
});

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(64.0),
        TextColor(Color::WHITE),
    )
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(32.0),
        TextColor(Color::WHITE),
    )
}
