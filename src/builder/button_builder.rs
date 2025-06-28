use crate::builder::node_builder::NodeBundle;
use crate::ui_plugin::color_plugin::{BackgroundColorChangeReactor, BorderColorChangeReactor};
use crate::ui_plugin::number_plugin::BorderSizeChangeReactor;
use bevy::prelude::*;

pub struct ButtonBuilder {
    border_radius: BorderRadius,
    justify_content: JustifyContent,
    align_items: AlignItems,
    interaction: Interaction,
    node_bundle: NodeBundle,
    text: Entity,
    border_color_change_reactor: Option<BorderColorChangeReactor>,
    border_size_change_reactor: Option<BorderSizeChangeReactor>,
    background_color_change_reactor: Option<BackgroundColorChangeReactor>,
}

impl ButtonBuilder {
    pub fn new(node_bundle: NodeBundle, text: Entity) -> Self {
        Self {
            border_radius: BorderRadius::MAX,
            interaction: Interaction::Pressed,

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            text,
            node_bundle,
            border_color_change_reactor: None,
            border_size_change_reactor: None,
            background_color_change_reactor: None,
        }
    }

    pub fn border_radius(mut self, border_radius: BorderRadius) -> Self {
        self.border_radius = border_radius;
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn interaction(mut self, interaction: Interaction) -> Self {
        self.interaction = interaction;
        self
    }

    pub fn border_color_change_reactor(mut self, source_id: &str) -> Self {
        self.border_color_change_reactor = Some(BorderColorChangeReactor {
            source_id: source_id.into(),
        });
        self
    }

    pub fn border_size_change_reactor(mut self, source_id: &str) -> Self {
        self.border_size_change_reactor = Some(BorderSizeChangeReactor {
            source_id: source_id.into(),
        });
        self
    }

    pub fn background_color_change_reactor(mut self, source_id: &str) -> Self {
        self.background_color_change_reactor = Some(BackgroundColorChangeReactor {
            source_id: source_id.into(),
        });
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let button = (
            Button,
            self.node_bundle,
            self.border_radius,
            self.interaction,
        );
        let parent = commands.spawn(button).id();
        commands.entity(parent).add_children(&[self.text]);
        if let Some(border_color_change_reactor) = self.border_color_change_reactor {
            commands.entity(parent).insert(border_color_change_reactor);
        }
        if let Some(border_size_change_reactor) = self.border_size_change_reactor {
            commands.entity(parent).insert(border_size_change_reactor);
        }
        if let Some(background_color_change_reactor) = self.background_color_change_reactor {
            commands
                .entity(parent)
                .insert(background_color_change_reactor);
        }
        parent
    }
}
