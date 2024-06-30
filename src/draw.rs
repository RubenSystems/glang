use eframe::{run_native, App};
use egui::Context;
use egui_graphs::{DefaultEdgeShape, DefaultNodeShape, Graph, GraphView};
use petgraph::{data::Build, graph::NodeIndex, stable_graph::StableGraph, Directed};

use crate::instructions::{Instruction, Value};

pub struct ProgramDraw {
    pub g: Graph<(), ()>,
}

pub fn generate_graph(prog: &Instruction) -> StableGraph<(), (), Directed> {
    let mut g = StableGraph::new();
    let parent_node = g.add_node(());

    match prog {
        Instruction::Out(v) => dfs_helper(&mut g, &parent_node, v),
    };

    g
}

fn dfs_helper(graph: &mut StableGraph<(), (), Directed>, parent: &NodeIndex, ins: &Value) {
    match ins {
        Value::ILoad(v) => {
            let iload_node: NodeIndex = graph.add_node(());
            let _ = graph.add_edge(iload_node, *parent, ());
        }
        Value::Add(a, b) => {
            let a_node = graph.add_node(());
            let b_node = graph.add_node(());

            dfs_helper(graph, &a_node, a);
            dfs_helper(graph, &b_node, b);
        }
    };
}

impl App for ProgramDraw {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut GraphView::<
                _,
                _,
                _,
                _,
                DefaultNodeShape,
                DefaultEdgeShape,
            >::new(&mut self.g));
        });
    }
}
