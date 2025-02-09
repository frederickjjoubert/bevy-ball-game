use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub const GAME_OVER_MENU_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    node.display = Display::Flex; // Hidden by Default
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node
};

pub const GAME_OVER_MENU_CONTAINER_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.display = Display::Flex;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(400.0);
    node.height = Val::Px(400.0);
    node.column_gap = Val::Px(8.0);
    node.row_gap = Val::Px(8.0);
    node
};

pub const BUTTON_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Px(200.0);
    node.height = Val::Px(80.0);
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(64.0),
        TextColor(Color::WHITE),
    )
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(48.0),
        TextColor(Color::WHITE),
    )
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont::from_font(asset_server.load("fonts/FiraSans-Bold.ttf")).with_font_size(32.0),
        TextColor(Color::WHITE),
    )
}
