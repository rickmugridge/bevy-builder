use bevy::color::palettes::css::WHITE;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct NodeBundle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}


pub struct NodeBuilder {
    bundle: NodeBundle,
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            bundle: NodeBundle::default(),
        }
    }
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn row(mut self, grid_template_rows: Vec<RepeatedGridTrack>) -> Self {
        self.bundle.node.display = Display::Grid;
        self.bundle.node.grid_template_rows = grid_template_rows;
        self
    }

    pub fn column(mut self, grid_template_columns: Vec<RepeatedGridTrack>) -> Self {
        self.bundle.node.display = Display::Grid;
        self.bundle.node.grid_template_columns = grid_template_columns;
        self
    }

    pub fn key_value_pairs(mut self) -> Self {
        self.bundle.node.display = Display::Grid;
        self.bundle.node.grid_template_columns = vec![GridTrack::flex(1.0), GridTrack::flex(1.0)];
        self.bundle.node.border = UiRect::all(Px(1.));
        self.bundle.border_color = BorderColor::from(WHITE);
        self.bundle.node.align_items = AlignItems::Stretch; // todo these don't help in layout on change
        self.bundle.node.justify_items = JustifyItems::Stretch;
        self.bundle.node.justify_content = JustifyContent::Stretch;
        self
    }

    pub fn text_field_node(mut self) -> Self {
        self.bundle.node.border = UiRect::all(Px(5.));
        self.bundle.border_color = BorderColor::from(WHITE);
        self.bundle.border_color = BorderColor::from(WHITE);
        self.bundle.node.margin = UiRect::all(Val::Px(5.0));
        self.bundle.background_color = WHITE.into();
        self
    }

    pub fn display(mut self, display: Display) -> Self {
        self.bundle.node.display = display;
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.bundle.node.overflow = overflow;
        self
    }

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.bundle.node.width = width;
        self.bundle.node.height = height;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.bundle.node.width = width;
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.bundle.node.height = height;
        self
    }

    pub fn bottom(mut self, bottom: Val) -> Self {
        self.bundle.node.bottom = bottom;
        self
    }

    pub fn right(mut self, right: Val) -> Self {
        self.bundle.node.right = right;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.bundle.node.margin = margin;
        self
    }

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.bundle.node.padding = padding;
        self
    }

    pub fn border(mut self, border: UiRect, border_color: Color) -> Self {
        self.bundle.node.border = border;
        self.bundle.border_color = BorderColor::from(border_color);
        self
    }

    pub fn border_of(mut self, val: Val, border_color: Color) -> Self {
        self.bundle.node.border = UiRect::all(val);
        self.bundle.border_color = BorderColor::from(border_color);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.bundle.border_color = BorderColor::from(border_color);
        self
    }

    pub fn background_color(mut self, background_color: Color) -> Self {
        self.bundle.background_color = BackgroundColor::from(background_color);
        self
    }

    pub fn position_type(mut self, position_type: PositionType) -> Self {
        self.bundle.node.position_type = position_type;
        self
    }

    pub fn position(mut self, left: Val, right: Val, top: Val, bottom: Val) -> Self {
        self.bundle.node.left = left;
        self.bundle.node.right = right;
        self.bundle.node.top = top;
        self.bundle.node.bottom = bottom;
        self
    }

    // Flex layout
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.bundle.node.flex_direction = direction;
        self
    }

    pub fn flex_wrap(mut self, wrap: FlexWrap) -> Self {
        self.bundle.node.flex_wrap = wrap;
        self
    }

    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.bundle.node.flex_grow = grow;
        self
    }

    pub fn flex_shrink(mut self, shrink: f32) -> Self {
        self.bundle.node.flex_shrink = shrink;
        self
    }

    pub fn flex_basis(mut self, basis: Val) -> Self {
        self.bundle.node.flex_basis = basis;
        self
    }

    // Alignment
    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.bundle.node.justify_content = justify_content;
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.bundle.node.align_items = align_items;
        self
    }

    // Gaps
    pub fn row_gap(mut self, gap: Val) -> Self {
        self.bundle.node.row_gap = gap;
        self
    }

    pub fn column_gap(mut self, gap: Val) -> Self {
        self.bundle.node.column_gap = gap;
        self
    }

    // Grid Layout
    pub fn grid_auto_flow(mut self, flow: GridAutoFlow) -> Self {
        self.bundle.node.grid_auto_flow = flow;
        self
    }

    pub fn grid_template_columns(mut self, template: Vec<RepeatedGridTrack>) -> Self {
        self.bundle.node.grid_template_columns = template;
        self
    }

    pub fn grid_template_rows(mut self, template: Vec<RepeatedGridTrack>) -> Self {
        self.bundle.node.grid_template_rows = template;
        self
    }

    pub fn grid_auto_rows(mut self, auto_rows: Vec<GridTrack>) -> Self {
        self.bundle.node.grid_auto_rows = auto_rows;
        self
    }

    pub fn grid_auto_columns(mut self, auto_columns: Vec<GridTrack>) -> Self {
        self.bundle.node.grid_auto_columns = auto_columns;
        self
    }

    pub fn grid_column(mut self, column: GridPlacement) -> Self {
        self.bundle.node.grid_column = column;
        self
    }

    pub fn grid_row(mut self, row: GridPlacement) -> Self {
        self.bundle.node.grid_row = row;
        self
    }

    // Convenience methods for grid layouts
    pub fn grid_span(mut self, column_span: u16, row_span: u16) -> Self {
        self.bundle.node.grid_column = GridPlacement::span(column_span);
        self.bundle.node.grid_row = GridPlacement::span(row_span);
        self
    }

    pub fn simple_grid(mut self, columns: u16, rows: u16) -> Self {
        self.bundle.node.grid_template_columns = vec![RepeatedGridTrack::flex(columns, 1.0)];
        self.bundle.node.grid_template_rows = vec![RepeatedGridTrack::flex(rows, 1.0)];
        self.bundle.node.display = Display::Grid;
        self
    }

    pub fn build(self) -> NodeBundle {
        self.bundle
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(self.bundle)
            .id()
    }
}
