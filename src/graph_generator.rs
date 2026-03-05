use eframe::egui;
use glam::*;
use rand::prelude::*;

use crate::Node;
use crate::gears;

// struct Node {
//     pos: glam::Vec2,
//     radius: f32,
// }

pub struct Graph {
    node_nbr: u16,
    node_colors: Vec<egui::Color32>,
    pub nodes: Vec<Node>,
    pub original_area_radius: f32,
    // pub node_area_radius: f32,
    // edges
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            node_nbr: 2,
            node_colors: vec![egui::Color32::LIGHT_RED, egui::Color32::LIGHT_BLUE],
            nodes: vec![
                Node {
                    pos: glam::vec2(-100.0, 0.0),
                    color: egui::Color32::LIGHT_BLUE,
                    text: String::from("A"),
                    radius: 20.0,
                },
                Node {
                    pos: glam::vec2(100.0, 0.0),
                    color: egui::Color32::LIGHT_RED,
                    text: String::from("B"),
                    radius: 20.0,
                },
            ],
            original_area_radius: 0.0,
        }
    }
}

impl Graph {
    pub fn new(node_nbr: u16, original_area_radius: f32, node_area_radius: f32) -> Self {
        Self {
            node_nbr,
            node_colors: vec![egui::Color32::LIGHT_RED, egui::Color32::LIGHT_BLUE],
            nodes: vec![],
            original_area_radius,
        }
    }

    pub fn generate_graph_nodes_chained(&mut self) {
        // Generate Base Graph Data Structure for Level
        // First Node is picked Randomly inside a circle
        self.nodes.clear();

        for i in 0..self.node_nbr {
            // Node Base Pos
            let mut pos: Vec2 = Vec2 { x: 0_f32, y: 0_f32 };

            if i > 0 {
                pos = self.nodes[(i - 1) as usize].pos;
            }

            // Node Circle Pos
            let mut rng = rand::rng();

            let radius: f32 = if i == 0 {
                self.original_area_radius
            } else {
                // rng.random_range(minimum_distance..100.0)
                rng.random_range(0.0..100.0)
            };
            let random_progress: f32 = rng.random_range(0.0..std::f32::consts::PI * 2.0);

            let node_pos = pos + gears::circle_pos(radius, random_progress);

            // Create Node
            let node = self.create_node(i, node_pos);
            self.nodes.push(node);
        }
    }

    pub fn generate_graph_nodes(&mut self) {
        self.nodes.clear();
        let mut rng = rand::rng();

        for i in 0..self.node_nbr {
            let radius: f32 = rng.random_range(0.0..self.original_area_radius);
            let random_progress: f32 = rng.random_range(0.0..std::f32::consts::PI * 2.0);

            let node_pos = gears::circle_pos(radius, random_progress);

            let node = self.create_node(i, node_pos);
            self.nodes.push(node);
        }
    }

    pub fn create_node(&self, id: u16, node_pos: Vec2) -> Node {
        let mut rng = rand::rng();
        let radius = rng.random_range(15.0..30.0);

        // Node Position -> Minimum distance between Nodes
        let mut adjusted_pos: Vec2 = node_pos;
        let mut overlapping_nodes = self.find_overlapping_nodes(&adjusted_pos, &radius);
        let mut count: u8 = 0;

        while count < 3 && overlapping_nodes.len() > 0 {
            adjusted_pos = self.adjust_node_pos(&overlapping_nodes, &adjusted_pos, &radius);
            overlapping_nodes = self.find_overlapping_nodes(&adjusted_pos, &radius);
            count += 1;
        }

        return Node {
            pos: adjusted_pos,
            color: self.node_colors[id as usize % self.node_colors.len()],
            text: String::from(id.to_string()),
            radius,
        };
    }

    fn find_overlapping_nodes(&self, start_pos: &Vec2, radius: &f32) -> Vec<&Node> {
        let mut overlapping_nodes: Vec<&Node> = vec![];

        for n in &self.nodes {
            let distance = start_pos.distance(n.pos);
            let min_distance = radius + n.radius;
            // println!("{distance} < {min_distance}");

            if distance < min_distance {
                overlapping_nodes.push(n);
            }
        }

        return overlapping_nodes;
    }

    fn adjust_node_pos(
        &self,
        overlapping_nodes: &Vec<&Node>,
        start_pos: &Vec2,
        radius: &f32,
    ) -> Vec2 {
        let mut final_direction = Vec2 { x: 0.0, y: 0.0 };
        let mut final_distance: f32 = 0.0;

        for n in overlapping_nodes {
            let distance = start_pos.distance(n.pos); // TODO : Avoid recalculating Distance
            let m_distance = radius + n.radius;
            // println!("{distance} < {minimum_distance}");

            if distance < m_distance {
                // let other_id = &n.text;
                // println!("Node {id} is too close to node {other_id}");

                final_direction += start_pos - n.pos; // Vector existing node -> created node
                final_distance += m_distance - distance;
            }
        }

        return start_pos + final_direction.normalize_or_zero() * final_distance;
    }
}
