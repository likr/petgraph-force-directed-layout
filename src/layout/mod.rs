use std::f32::consts;
use petgraph::graph::Graph;
use petgraph::visit::GetAdjacencyMatrix;
use super::graph::{Node, Edge};

#[derive(Clone)]
struct NodePosition {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn jiggle() -> f32 {
    0.00001
}

fn distance(p1: &NodePosition, p2: &NodePosition) -> f32 {
    let x = p1.x - p2.y;
    let y = p1.y - p2.y;
    (x * x + y * y).sqrt()
}

fn initialize_nodes(n: usize) -> Vec<NodePosition> {
    let initial_radius = 10.0;
    let initial_angle = consts::PI * (3.0 - (5.0 as f32).sqrt());
    let mut positions = Vec::new();
    for i in 0..n {
        let radius = initial_radius * (i as f32).sqrt();
        let angle = (i as f32) * initial_angle;
        positions.push(NodePosition {
            x: radius * angle.cos(),
            y: radius * angle.sin(),
            vx: 0.0,
            vy: 0.0,
        });
    }
    positions
}

fn apply_charge(alpha: f32, positions: &mut Vec<NodePosition>) {
}

fn apply_link(graph: &Graph<Node, Edge>, alpha: f32, positions: &mut Vec<NodePosition>) {
    let distance = 30.0;
    let strength = 0.5;
    let bias = 0.5;
    for u in graph.node_indices() {
        let i = u.index();
        let pos_u = positions[i].clone();
        for (v, _) in graph.edges(u) {
            let j = v.index();
            if j < i {
                continue;
            }
            let pos_v = positions[j].clone();
            let x = pos_v.x + pos_v.vx - pos_u.x - pos_u.vx;
            let y = pos_v.y + pos_v.vy - pos_u.y - pos_u.vy;
            let x = if x == 0.0 { jiggle() } else { x };
            let y = if y == 0.0 { jiggle() } else { y };
            let l = (x * x + y * y).sqrt();
            let l = (l - distance) / l * alpha * strength;
            let x = x * l;
            let y = y * l;
            {
                let ref mut pos_v = positions[j];
                pos_v.vx -= x * bias;
                pos_v.vy -= y * bias;
            }
            {
                let ref mut pos_u = positions[i];
                pos_u.vx += x * (1.0 - bias);
                pos_u.vy += y * (1.0 - bias);
            }
        }
    }
}

pub fn layout(graph: Graph<Node, Edge>) {
    let n = graph.node_count();
    let mut positions = initialize_nodes(n);
    let mut alpha  = 1.0;
    let alpha_min : f32 = 0.001;
    let alpha_target = 0.0;
    let alpha_decay = 1.0 - alpha_min.powf(1.0 / 300.0);
    let velocity_decay = 0.6;
    while alpha >= alpha_min {
        alpha += (alpha_target - alpha) * alpha_decay;
        apply_charge(alpha, &mut positions);
        apply_link(&graph, alpha, &mut positions);
        for u in graph.node_indices() {
            let ref mut pos_u = positions[u.index()];
            pos_u.vx *= velocity_decay;
            pos_u.vy *= velocity_decay;
            pos_u.x += pos_u.vx;
            pos_u.y += pos_u.vy;
        }
    }
    for i in 0..n {
        let ref p = positions[i];
        println!("({}, {})", p.x, p.y);
    }
}
