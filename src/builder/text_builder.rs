use crate::builder::node_builder::NodeBundle;
use crate::edit_plugin::text_edit_plugin::{TextColorReactor, TextContentReactor};
use bevy::prelude::*;
use bevy::ui::ContentSize;
use bevy::ui::widget::TextNodeFlags;

#[derive(Bundle, Default)]
pub struct TextBundle {
    pub text: Text,
    pub font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
    pub text_shadow: TextShadow,
    pub flags: TextNodeFlags,
    pub content_size: ContentSize,
    pub inner_node_bundle: NodeBundle,
    text_content_reactor: TextContentReactor,
    text_color_reactor: TextColorReactor,
}

pub struct TextBuilder {
    bundle: TextBundle,
    outer_node_bundle: NodeBundle,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            bundle: TextBundle {
                text_shadow: TextShadow {
                    color: Color::NONE,
                    offset: Vec2::ZERO,
                },
                ..default()
            },
            outer_node_bundle: NodeBundle::default(),
        }
     }

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.bundle.text = Text::new(content.into());
        self
    }
    
    pub fn text_color(mut self, text_color: Color) -> Self {
        self.bundle.text_color = TextColor(text_color);
        self
    }

    pub fn font(mut self, font: TextFont) -> Self {
        self.bundle.font = font;
        self
    }

    pub fn inner_node_bundle(mut self, inner_node_bundle: NodeBundle) -> Self {
        self.bundle.inner_node_bundle = inner_node_bundle;
        self
    }

    pub fn outer_node_bundle(mut self, outer_node_bundle: NodeBundle) -> Self {
        self.outer_node_bundle = outer_node_bundle;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.bundle.text_color = TextColor(color);
        self
    }

    pub fn justify_text(mut self, text_layout: TextLayout) -> Self {
        self.bundle.text_layout = text_layout;
        self
    }

    pub fn text_shadow(mut self, text_shadow: TextShadow) -> Self {
        self.bundle.text_shadow = text_shadow;
        self
    }

    pub fn text_content_reactor(mut self, source_id: &str) -> Self {
        self.bundle.text_content_reactor = TextContentReactor::Active {
            source_id: source_id.into(),
        };
        self
    }

    pub fn text_color_reactor(mut self, source_id: &str) -> Self {
        self.bundle.text_color_reactor = TextColorReactor::Active {
            source_id: source_id.into(),
        };
        self
    }

    pub fn bundle(self) -> TextBundle {
        self.bundle
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let outer = commands.spawn(self.outer_node_bundle).id();
        let text_entity = commands.spawn(self.bundle).id();
        commands.entity(outer).add_child(text_entity);
        outer
    }
}
