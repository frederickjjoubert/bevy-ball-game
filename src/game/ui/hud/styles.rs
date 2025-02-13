use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);

pub static HUD_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    width: Val::Percent(100.0),
    height: Val::Percent(15.0),
    ..default()
});

pub static LHS_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Percent(80.0),
    margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
    ..default()
});

pub static RHS_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Percent(80.0),
    margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
    ..default()
});

pub static IMAGE_NODE: std::sync::LazyLock<Node> = std::sync::LazyLock::new(|| Node {
    width: Val::Px(48.0),
    height: Val::Px(48.0),
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..default()
});

pub fn get_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(64.0),
        TextColor(Color::WHITE),
    )
}
