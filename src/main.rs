mod gears;
mod graph_generator;

use eframe::egui;
use rand::prelude::*;

use crate::graph_generator::Graph;

fn main() -> eframe::Result {
    // return start_app(LevelGraphApp::default())

    let mut graph_data = Graph::new(10, 300.0, 40.0);
    graph_data.generate_graph_nodes();
    return start_app(build_graph_from_data(graph_data));
}

fn start_app(app: LevelGraphApp) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Level Graph Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

fn rng_test() {
    let mut rng = rand::rng();
    let x: f32 = rng.random_range(0.0..100.0);
    let y: i32 = rng.random_range(1..10);
    println!("RNG 1 : {x}, RNG 2 : {y}")
}

fn build_graph_from_data(graph_data: Graph) -> LevelGraphApp {
    return LevelGraphApp {
        label: "Display Graph Data".to_owned(),
        graph_data,
    };
}

struct Node { // TODO : Rename NodeGraph
    pos: glam::Vec2,
    color: egui::Color32,
    text: String,
    radius: f32,
}

// TODO : Add Edge struct

struct LevelGraphApp {
    label: String,
    graph_data: Graph,
}

impl LevelGraphApp {
    fn draw_edges(
        &self,
        i: usize,
        response: &egui::Response,
        painter: &egui::Painter,
        node_pos: egui::Pos2,
    ) {
        // Next Node Exist -> Draw Edge (Line)
        if i < self.graph_data.nodes.len() - 1 {
            let next_node = &self.graph_data.nodes[i + 1];
            let next_node_pos =
                response.rect.center() + egui::vec2(next_node.pos.x, next_node.pos.y);

            painter.line_segment(
                [node_pos, next_node_pos],
                egui::Stroke::new(2.0, egui::Color32::GRAY),
            );
        }
    }

    fn draw_minimum_range_circles(&self, painter: &egui::Painter, node_pos: &egui::Pos2, color: &egui::Color32) {
        painter.circle(
            *node_pos,
            self.graph_data.node_area_radius,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, *color),
        );
    }
}

impl Default for LevelGraphApp {
    fn default() -> Self {
        Self {
            label: "Display Graph Data".to_owned(),
            graph_data: Graph::default(),
        }
    }
}

impl eframe::App for LevelGraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.label);

            // This is where you will eventually draw your graph nodes
            // ui.label("The graph will be rendered here.");

            if ui.button("Generate").clicked() {
                println!("Button clicked!");
                rng_test();
                self.graph_data.generate_graph_nodes();
            }

            // Create Graph Area
            let (response, painter) = ui.allocate_painter(
                ui.available_size(), // Use all remaining space
                egui::Sense::hover(),
            );

            let node_painter = painter
                .clone()
                .with_layer_id(egui::LayerId::new(egui::Order::Background, response.id));

            // Draw Nodes
            for i in 0..self.graph_data.nodes.len() {
                // Node Circle
                let node: &Node = &self.graph_data.nodes[i];
                let node_pos = response.rect.center() + egui::vec2(node.pos.x, node.pos.y); // Converte glam::Vec2 -> egui::Vec2

                node_painter.circle_filled(node_pos, node.radius, node.color);

                // Node Label
                node_painter.text(
                    node_pos,
                    egui::Align2::CENTER_CENTER,
                    &node.text,
                    egui::FontId::proportional(15.0),
                    egui::Color32::BLACK,
                );

                // self.draw_edges(i, &response, &painter, node_pos);

                self.draw_minimum_range_circles(&painter, &node_pos, &node.color);

                // Draw Area Circle
                painter.circle(
                    response.rect.center(),
                    self.graph_data.original_area_radius,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke::new(2.0, egui::Color32::GRAY),
                );
            }
        });
    }
}
