use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const HUD_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(15.0);
    style.margin = UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0));

    style
};

pub const LHS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Percent(80.0);
    style.margin = UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0));

    style
};

pub const RHS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Percent(80.0);
    style.margin = UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0));

    style
};

pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));

    style
};

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}
