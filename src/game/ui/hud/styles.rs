use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const HUB_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(48.0), Val::Px(48.0)),
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub fn hub_card_node_bundle(margin_left: f32, margin_right: f32) -> NodeBundle {
    return NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
            margin: UiRect::new(
                Val::Px(margin_left),
                Val::Px(margin_right),
                Val::Px(0.0),
                Val::Px(0.0),
            ),
            ..default()
        },
        background_color: BACKGROUND_COLOR.into(),
        ..default()
    };
}
