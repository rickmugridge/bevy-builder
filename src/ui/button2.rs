use crate::builder::button_builder::ButtonBuilder;
use crate::builder::node_builder::NodeBuilder;
use crate::builder::text_builder::TextBuilder;
use crate::builder::text_font_builder::TextFontBuilder;
use bevy::color::palettes::basic::RED;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct ButtonPlugin2;

impl Plugin for ButtonPlugin2 {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);
    spawn_button(&mut commands, &assets);
}

fn spawn_button(commands: &mut Commands, asset_server: &AssetServer) {
    let container = NodeBuilder::new()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .build_and_spawn(commands);

    let text_font = TextFontBuilder::new()
        .load_font(asset_server, "fonts/FiraSans-Bold.ttf")
        .font_size(33.)
        .build();

    let text = TextBuilder::new()
        .content("Button")
        .font(text_font)
        .color(Color::srgb(0.9, 0.9, 0.9))
        .shadow(Color::BLACK, Vec2::new(1.0, 1.0))
        .build_and_spawn(commands);

    let button_node = NodeBuilder::new()
        .width(Val::Px(150.0))
        .height(Val::Px(65.0))
        .border(UiRect::all(Val::Px(5.0)))
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .build();

    let button = ButtonBuilder::new(button_node, text)
        .id("b")
        .background_color(NORMAL_BUTTON)
        .border_color(Color::BLACK)
        .build_and_spawn(commands);

    commands.entity(container).add_children(&[button]);
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
