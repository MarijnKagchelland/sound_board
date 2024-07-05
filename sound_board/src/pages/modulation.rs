use egui::style::Interaction;
use egui::{
    Pos2,
    Frame, 
    Sense,
    Button, 
    Context, 
    TextEdit,
    SidePanel,
    CursorIcon, 
    CentralPanel, 
};
use egui_graphs::{
    Graph, 
    GraphView, 
    SettingsStyle,
    DefaultEdgeShape, 
    DefaultNodeShape,
    SettingsNavigation,
    SettingsInteraction, 
};
use petgraph::csr::IndexType;
use petgraph::stable_graph::{
    DefaultIx, 
    EdgeIndex,
    NodeIndex, 
    StableGraph, 
    StableUnGraph, 
};
use petgraph::Undirected;

use crate::log_system::*;
use crate::pages::Page;
use crate::audio::SoundBoard;

use crate::pages::custom_graph::CustomNode;

pub struct Modulation {
    graph:          Graph<(),()>,

    style:          StyleSettings,
    interaction:    InteractionSettings,
    navigation:     NavigationSettings,

    label_input:    String,
    selected_node:  Option<NodeIndex>,
    selected_edge:  Option<EdgeIndex>,
}

impl Modulation {
    fn init() -> Self {
        Self {
            graph:          Graph::from(&base_graph()),

            style:          StyleSettings::init(),
            interaction:    InteractionSettings::init(),
            navigation:     NavigationSettings::init(),

            label_input:    String::default(),
            selected_node:  Option::default(),
            selected_edge:  Option::default(),
        }
    }

    fn read_data(&mut self) {
        if !self.graph.selected_nodes().is_empty() {
            let index = self.graph.selected_nodes().first().unwrap();
            self.selected_node  = Some(*index);
            self.selected_edge  = None;
            self.label_input    = self.graph
                .node(*index)
                .unwrap()
                .label();
        }
        if !self.graph.selected_edges().is_empty() {
            let index = self.graph.selected_edges().first().unwrap();
            self.selected_edge  = Some(*index);
            self.selected_node  = None;
            self.label_input    = self.graph
                .edge(*index)
                .unwrap()
                .label();
        }
    }

    fn render(&mut self, ctx: &Context) {
        SidePanel::right("right_panel").show(ctx, |ui| {
            ui.label("Change label");
            ui.add_enabled_ui(
                self.selected_node.is_some() || self.selected_edge.is_some(),
                |ui| {
                    TextEdit::singleline(&mut self.label_input)
                        .hint_text("Select node or edge")
                        .show(ui);
                }
            );
            if ui.button("Reset").clicked() {
                self.reset();
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut GraphView::<
                _,
                _,
                _,
                _,
                DefaultNodeShape,
                DefaultEdgeShape,
                >::new(&mut self.graph)
                    .with_navigations(self.navigation.get_settings())
                    .with_interactions(self.interaction.get_settings())                    
                    .with_styles(self.style.get_settings())
            );
        });
    }
    
    fn update_data(&mut self) {
        if self.selected_node.is_some() && self.selected_edge.is_some() {
            return;
        }
        if self.selected_node.is_some() {
            let index = self.selected_node.unwrap();
            if index.index().to_string() == self.label_input {
                return;
            }
            self.graph
                .node_mut(index)
                .unwrap()
                .set_label(self.label_input.clone());
        }
        if self.selected_edge.is_some() {
            let index = self.selected_edge.unwrap();
            if index.index().to_string() == self.label_input {
                return;
            }
            self.graph
                .edge_mut(index)
                .unwrap()
                .set_label(self.label_input.clone());
        }
    }

    fn reset(&mut self) {
        self.selected_edge = None;
        self.selected_node = None;
        self.graph = Graph::from(&base_graph());
    }
}

impl Page for Modulation {
    fn show(&mut self, ctx: &Context, audio_board: &mut SoundBoard) {
        self.read_data();
        self.render(&ctx);
        self.update_data();
    }
}

impl Default for Modulation {
    fn default() -> Self {
        Self::init()
    }
}


fn base_graph() -> StableGraph<(), ()> {
    let mut graph = StableGraph::new();

    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());
    return graph;
}

struct StyleSettings {
    style:  SettingsStyle 
}

impl StyleSettings {
    pub fn init() -> Self {
        let style = SettingsStyle::default()
            .with_labels_always(true);
        Self {
            style
        }
    }

    pub fn get_settings(&self) -> &SettingsStyle {
        &self.style
    }
}

struct NavigationSettings {
    navi:  SettingsNavigation 
}

impl NavigationSettings {
    pub fn init() -> Self {
        let navi = SettingsNavigation::default()
            .with_fit_to_screen_enabled(false)
            .with_zoom_and_pan_enabled(true);
        Self {
            navi
        }
    }

    pub fn get_settings(&self) -> &SettingsNavigation {
        &self.navi
    }
}

struct InteractionSettings {
    interaction:  SettingsInteraction 
}

impl InteractionSettings {
    pub fn init() -> Self {
        let interaction = SettingsInteraction::default()
            .with_dragging_enabled(true)
            .with_node_selection_enabled(true)
            .with_edge_selection_enabled(true);
        Self {
            interaction
        }
    }

    pub fn get_settings(&self) -> &SettingsInteraction {
        &self.interaction
    }
}

