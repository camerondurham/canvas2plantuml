use clap::Parser;
use jsoncanvas::edge::Edge;
use jsoncanvas::{EdgeId, JsonCanvas, Node, NodeId};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/// Reads a JSON file and loads it into a HashMap
#[derive(Parser, Debug)]
struct Args {
    /// Path to the JSON file
    #[arg(required = true)]
    json_file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut json_file = match File::open(&args.json_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening JSON file: {}", err);
            std::process::exit(1);
        }
    };

    let mut json_contents = String::new();
    if let Err(err) = json_file.read_to_string(&mut json_contents) {
        eprintln!("Error reading JSON file: {}", err);
        std::process::exit(1);
    }

    let json_data: Value = match serde_json::from_str(&json_contents) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            std::process::exit(1);
        }
    };

    // See full usage examples at https://docs.rs/jsoncanvas/latest/jsoncanvas/
    let canvas: JsonCanvas = json_data.to_string().parse().unwrap();
    let s = canvas.to_string();
    println!("JsonCanvas: {}", s);

    let mut plantuml_lines: Vec<String> = vec!["left to right direction".to_string()];

    let nodes: &HashMap<NodeId, Node> = canvas.get_nodes();
    nodes.iter().for_each(|n| {
        let (id, node) = n;
        match node {
            Node::Text(tn) => {
                plantuml_lines.push(format!("rectangle \"{}\" as {}", tn.text(), id));
            }
            _ => {
                dbg!("Skip processing of non-text node");
            }
        }
    });

    let edges: &HashMap<EdgeId, Edge> = canvas.get_edges();
    edges.iter().for_each(|e| {
        // TODO: implement right/down/up/left/right arrows
        let (_, edge) = e;
        plantuml_lines.push(format!("{} --> {}", edge.from_node, edge.to_node));
    });
    
    println!("{}", plantuml_lines.join("\n"));
}
