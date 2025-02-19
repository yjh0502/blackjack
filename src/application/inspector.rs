use crate::{graph::graph_editor_egui::editor_state::GraphEditorState, prelude::*};
use egui::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InspectorTab {
    Properties,
    Spreadsheet,
}

pub struct InspectorTabs {
    current_view: InspectorTab,
    properties: PropertiesTab,
    spreadsheet: SpreadsheetTab,
}

impl InspectorTabs {
    pub fn new() -> Self {
        Self {
            current_view: InspectorTab::Properties,
            properties: PropertiesTab {},
            spreadsheet: SpreadsheetTab {
                current_view: SpreadsheetViews::Vertices,
            },
        }
    }
}

impl Default for InspectorTabs {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PropertiesTab {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpreadsheetViews {
    Vertices,
    Halfedges,
    Faces,
}

pub struct SpreadsheetTab {
    pub current_view: SpreadsheetViews,
}

impl InspectorTabs {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        mesh: Option<&HalfEdgeMesh>,
        editor_state: &mut GraphEditorState,
    ) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.current_view,
                InspectorTab::Properties,
                "Inspector",
            );
            ui.selectable_value(
                &mut self.current_view,
                InspectorTab::Spreadsheet,
                "Spreadsheet",
            );
        });
        ui.separator();
        match self.current_view {
            InspectorTab::Properties => self.properties.ui(ui, editor_state),
            InspectorTab::Spreadsheet => self.spreadsheet.ui(ui, mesh),
        }
    }
}

pub fn tiny_checkbox(ui: &mut Ui, value: &mut bool) {
    let mut child_ui = ui.child_ui(ui.available_rect_before_wrap(), *ui.layout());
    child_ui.spacing_mut().icon_spacing = 0.0;
    child_ui.spacing_mut().interact_size = egui::vec2(16.0, 16.0);
    child_ui.checkbox(value, "");
    ui.add_space(24.0);
}

impl PropertiesTab {
    fn ui(&self, ui: &mut Ui, editor_state: &mut GraphEditorState) {
        let graph = &mut editor_state.graph;
        if let Some(node) = editor_state.selected_node {
            let node = &graph[node];
            let inputs = node.inputs.clone();
            ui.vertical(|ui| {
                for (param_name, param) in inputs {
                    if graph.connection(param).is_some() {
                        ui.label(param_name);
                    } else {
                        ui.horizontal(|ui| {
                            tiny_checkbox(ui, &mut graph[param].shown_inline);
                            graph[param].value_widget(&param_name, ui);
                        });
                    }
                }
            });
        } else {
            ui.label("No node selected. Click a node's title to select it.");
        }
    }
}
impl SpreadsheetTab {
    fn ui(&mut self, ui: &mut Ui, mesh: Option<&HalfEdgeMesh>) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.current_view,
                SpreadsheetViews::Vertices,
                "Vertices",
            );
            ui.selectable_value(&mut self.current_view, SpreadsheetViews::Faces, "Faces");
            ui.selectable_value(
                &mut self.current_view,
                SpreadsheetViews::Halfedges,
                "Half edges",
            );
        });

        if let Some(mesh) = mesh {
            let scroll_area = ScrollArea::both().auto_shrink([false, false]);
            scroll_area.show(ui, |ui| match self.current_view {
                SpreadsheetViews::Vertices => {
                    // Vertex spreadsheet
                    Grid::new("vertex-spreadsheet")
                        .striped(true)
                        .num_columns(4)
                        .show(ui, |ui| {
                            ui.label("");
                            ui.label("x");
                            ui.label("y");
                            ui.label("z");
                            ui.end_row();

                            for (idx, (_, v)) in mesh.iter_vertices().enumerate() {
                                ui.label(idx.to_string());
                                ui.monospace(format!("{: >6.3}", v.position.x));
                                ui.monospace(format!("{: >6.3}", v.position.y));
                                ui.monospace(format!("{: >6.3}", v.position.z));
                                ui.end_row();
                            }
                        })
                }
                SpreadsheetViews::Halfedges => {
                    // Halfedge spreadsheet
                    Grid::new("halfedge-spreadsheet")
                        .striped(true)
                        .num_columns(1)
                        .show(ui, |ui| {
                            ui.label("");
                            ui.end_row();

                            for (idx, _) in mesh.iter_halfedges().enumerate() {
                                ui.label(idx.to_string());
                                ui.end_row();
                            }
                        })
                }
                SpreadsheetViews::Faces => {
                    // Face spreadsheet
                    Grid::new("halfedge-spreadsheet")
                        .striped(true)
                        .num_columns(1)
                        .show(ui, |ui| {
                            ui.label("");
                            ui.end_row();

                            for (idx, _) in mesh.iter_faces().enumerate() {
                                ui.label(idx.to_string());
                                ui.end_row();
                            }
                        })
                }
            });
        }
    }
}
