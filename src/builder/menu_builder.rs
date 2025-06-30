use crate::builder::node_builder::NodeBundle;
use bevy::prelude::{Commands, Component, Entity};
use bevy::ui::Display;

#[derive(Component, Default)]
pub struct Menu {

}

pub struct MenuBuilder {
    menus: Vec<(String, Entity)>,
    node_bundle: NodeBundle,
    columnar: bool,
}

impl MenuBuilder {
    pub fn new() -> Self {
        Self {
            menus: Vec::new(),
            node_bundle: NodeBundle::default(),
            columnar: true,
        }
    }

    pub fn add_menus(mut self, menus: Vec<(String, Entity)>) -> Self {
        self.menus = menus;
        self
    }

    pub fn node(mut self, node: NodeBundle) -> Self {
        self.node_bundle = node;
        self
    }

    pub fn columnar(mut self, columnar:bool) -> Self {
        self.columnar = columnar;
        self
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let container = self.make_container(commands);
        container
    }

    fn make_container(mut self, commands: &mut Commands) -> Entity {
        self.node_bundle.node.display = Display::Grid;
        if self.columnar {
        } else {

        }
        commands
            .spawn(self.node_bundle)
            .id()  }
}