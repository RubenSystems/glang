use draw::{generate_graph, ProgramDraw};
use instructions::{Instruction, Value};

mod draw;
mod instructions;
// fn main() {

//     ins.execute();
// }

use eframe::{run_native, App};
use egui::Context;
use egui_graphs::{DefaultEdgeShape, DefaultNodeShape, Graph, GraphView};
use petgraph::{data::Build, stable_graph::StableGraph, Directed};

fn main() {
    let ins = Instruction::Out(Value::Add(
        Box::new(Value::ILoad(1)),
        Box::new(Value::ILoad(2)),
    ));

    let mut g = generate_graph(&ins);

    let app = ProgramDraw { g: Graph::from(&g) };

    let native_options = eframe::NativeOptions::default();
    run_native("Program", native_options, Box::new(|cc| Box::new(app))).unwrap();
}
