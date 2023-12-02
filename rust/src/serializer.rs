use serde::Serialize;

use super::*;

#[derive(Serialize)]
struct Data {
    root: Node,
    edges: Vec<Edge>,
}

#[derive(Serialize, Default)]
struct Node {
    name: String, 
    value: usize,
    children: Vec<Node>,
}

#[derive(Serialize)]
struct Edge {
    src: String,
    dst: String,
}

/// Convert into sigma.js's graph format, and serialize.
pub fn serialize<'a> (path: &'a str, graph: Graph) 
{
    let mut root = Node::default();
    root.name = "global".to_owned();
    graph.v
        .iter()
        .for_each(|func| {
            let name  = func.signature.to_string();
            let class = func.signature.class.clone();
            let node  = Node {
                name,
                value: 15,
                children: vec![],
            };

            if let Some(class) = class {
                if let Some(class) = root.children.iter_mut().find(|node| node.name == class) {
                    class.children.push(node);
                } else {
                    let class = Node {
                        name: class,
                        value: 0,
                        children: vec![node]
                    };
                    root.children.push(class);
                }
            } else {
                root.children.push(node);
            }
        });

    let edges = graph.e
        .iter()
        .map(|edge| {
            Edge {
                src: graph.v[edge.0].signature.to_string(),
                dst: graph.v[edge.1].signature.to_string(),
            }
        })
        .collect();

    let data = Data { root, edges };
    let serialized = serde_json::to_string(&data).unwrap();

    fs::write(path, serialized)
        .expect(&format!("Unable to write to file '{}'", path));
}
