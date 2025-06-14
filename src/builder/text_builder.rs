use bevy::prelude::*;
use bevy::ui::ContentSize;
use bevy::ui::widget::TextNodeFlags;

#[derive(Debug, Component)]

pub struct TextBuilder {
    content: String,
    font: TextFont,
    color: Color,
    justify_text: JustifyText,
    linebreak: LineBreak,
    shadow_color: Color,
    shadow_offset: Vec2,
    node: Node,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            content: "Default".to_string(),
            font: TextFont::default(),
            color: Color::WHITE,
            justify_text: JustifyText::default(),
            linebreak: LineBreak::default(),
            shadow_color: Color::NONE,
            shadow_offset: Vec2::ZERO,
            node: Node::default(),
        }
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

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let text = (
            Text::new(self.content),
            self.font,
            TextColor(self.color),
            TextLayout::new(self.justify_text, self.linebreak),
            TextShadow {
                color: self.shadow_color,
                offset: self.shadow_offset,
            },
            TextNodeFlags::default(),
            ContentSize::default(),
            self.node,
        );
        commands.spawn(text).id()
    }
}
