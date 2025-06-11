use crate::builder::box_builder::BoxBuilder;
use bevy::prelude::*;
use bevy::ui::widget::TextNodeFlags;
use bevy::ui::ContentSize;

#[derive(Debug, Component, Default)]
#[require(Interaction)]
pub struct TextField {
    id: String,
    pub cursor_on:bool
}

pub struct TextFieldBuilder {
    text_field: TextField,
    content: String,
    font: TextFont,
    color: Color,
    justify_text: JustifyText,
    linebreak: LineBreak,
    shadow_color: Color,
    shadow_offset: Vec2,
    border_color: BorderColor,
    node: Node,
}

impl TextFieldBuilder {
    pub fn new() -> Self {
        Self {
            text_field: TextField::default(),
            content: "Default".to_string(),
            font: TextFont::default(),
            color: Color::BLACK,
            justify_text: JustifyText::default(),
            linebreak: LineBreak::default(),
            shadow_color: Color::NONE,
            shadow_offset: Vec2::ZERO,
            border_color: BorderColor::from(Color::WHITE),
            node: Node::default(),
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.text_field.id = id.into();
        self
    }

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = content.into();
        self
    }

    pub fn font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }

    pub fn node(mut self, node: Node) -> Self {
        self.node = node;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn justify_text(mut self, justify_text: JustifyText) -> Self {
        self.justify_text = justify_text;
        self
    }

    pub fn linebreak(mut self, linebreak: LineBreak) -> Self {
        self.linebreak = linebreak;
        self
    }

    pub fn shadow(mut self, color: Color, offset: Vec2) -> Self {
        self.shadow_color = color;
        self.shadow_offset = offset;
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = BorderColor::from(border_color);
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let text = (
            Text::new(self.content),
            self.text_field,
            self.border_color,
            self.font,
            TextColor(self.color),
            TextLayout::new(self.justify_text, self.linebreak),
            TextShadow {
                color: self.shadow_color,
                offset: self.shadow_offset,
            },
            TextNodeFlags::default(),
            ContentSize::default(),
            BackgroundColor(Color::WHITE), // This has no impact because text_field_hover() sets it as WHITE on first, auto, Update
            self.node,
        );
        let border_node = BoxBuilder::new()
            .border_color(Color::WHITE)
            .border_of(Val::Px(1.))
            .build_and_spawn(commands);
        let text_entity = commands.spawn(text).id();
        commands.entity(border_node).add_child(text_entity);
        border_node
    }
}
