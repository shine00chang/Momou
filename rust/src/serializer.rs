use serde::Serialize;

use super::*;


#[derive(Serialize)]
struct Data {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

#[derive(Serialize)]
struct Node {
    key: usize, 
    attributes: Attributes,
}

#[derive(Serialize)]
struct Attributes {
    label: String,
    size: usize,
    x: usize,
    y: usize,
    class: String,
}

#[derive(Serialize)]
struct Edge {
    key: usize, 
    source: usize,
    target: usize,
}

/// Convert into sigma.js's graph format, and serialize.
pub fn serialize<'a> (path: &'a str, graph: Graph) 
{
    let nodes = graph.v
        .iter()
        .enumerate()
        .map(|(i, func)| {
            let label = func.signature.to_string();
            Node {
                key: i,
                attributes: Attributes { 
                    label,
                    size: 15,
                    x: 12,
                    y: 214,
                    class: func.signature.class.clone().unwrap_or_else(|| "global".to_owned())
                }
            }
        })
        .collect();
    let edges = graph.e
        .iter()
        .enumerate()
        .map(|(i, edge)| {
            Edge {
                key: i,
                source: edge.0,
                target: edge.1,
            }
        })
        .collect();
    let data = Data { nodes, edges };
    let serialized = serde_json::to_string(&data).unwrap();

    fs::write(path, serialized)
        .expect(&format!("Unable to write to file '{}'", path));
}
