use tree_sitter_tags::{TagsContext, TagsConfiguration, Tag};
use serde::Serialize;

use std::collections::{HashSet, HashMap};
use std::env;
use std::fs;
use std::ops::Range;
use std::path::Path;

pub const RST: &str = "\x1b[0m";
pub const BLD: &str = "\x1b[1m";
pub const HLT: &str = "\x1b[48;5;226m";


#[derive(Debug, Clone, Default)]
struct Func<'a> {
    signature: Signature<'a>, 
    range: Range<usize>,
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
struct Signature<'a> {
    name: &'a str,
    class: Option<&'a str>
}
impl<'a> Signature<'a> {
    fn to_string (&self) -> String {
        if let Some(class) = self.class {
            let mut s = String::from(class);
            s.push_str("::");
            s.push_str(self.name);
            s
        } else {
            String::from(self.name)
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Annotation<'a> {
    identifier: &'a str,
    class: &'a str,
}

fn main () {
    println!("=== {BLD} Hello, testing tree-sitter-tags {RST} ===");

    // Get Input
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        panic!("{BLD}did not provide input file{RST}");
    }

    println!("Using file '{}'", args[1]);
    let path = Path::new(&args[1]);
    if path.is_dir() {
        panic!("{BLD}input was a directory{RST}");
    }
    let output_path = "./data.json";

    // Read file
    let file = std::fs::read_to_string(path).expect("{BLD}failed to open file{RST}");

    // Get tags
    //let (funcs, classes, invokes, annos) = get_tags(&file);
    let tags = get_tags(&file);

    let funcs = make_funcs(&file, &tags);
    // let funcs = find_side_effects(&file, funcs, &tags);
    let graph = make_graph(&file, funcs, &tags);
    serialize(output_path, graph);
}

/// Uses Treesitter to extract tags
fn get_tags (file: &str) -> Vec<(String, Tag)>
{
    // Read query schematic file.
    const TAG_QUERY_FILE: &str = "tags.scm";
    let tagging_query = std::fs::read_to_string(&TAG_QUERY_FILE)
        .expect(&format!("{BLD}failed to open tagging query file '{}'{RST}", &TAG_QUERY_FILE));

    // TS context
    let mut context = TagsContext::new();
    let config = TagsConfiguration::new(
        tree_sitter_javascript::language(),
        &tagging_query,
        tree_sitter_javascript::LOCALS_QUERY)
        .unwrap();

    /*let (funcs, classes, invokes, annos)*/
    let v = context.generate_tags(
        &config,
        &file.as_bytes(),
        None)
        .unwrap()
        .0
        .map(|x| {
            let tag = x.unwrap();
            (
                config.syntax_type_name(tag.syntax_type_id).to_owned(),
                tag
            )
        })
        .collect();
    v
}

/// Converts function tags into Func objects. 
/// Assigns class membership to function.
fn make_funcs<'a> (file: &'a str, tags: &'a Vec<(String, Tag)>) -> Vec<Func<'a>> 
{
    let classes = tags
        .iter()
        .filter(|(t, _)| *t == "class")
        .map(|(_, tag)| tag);
    tags
        .iter()
        .filter(|(t, _)| *t == "function" || *t == "method")
        .map(|(_, func)| {
            let class = classes
                .clone()
                .find(|class| class.range.contains(&func.range.start))
                .map(|class| &file[class.name_range.clone()]);
            Func {
                signature: Signature { 
                    name: &file[func.name_range.clone()], 
                    class },
                range: func.range.clone(),
            }
        })
        .collect()
}

struct Graph<'a> {
    v: Vec<Func<'a>>,
    m: HashMap<Signature<'a>, usize>,
    e: HashSet<(usize, usize)>
}
impl<'a> Graph<'a> {
    fn from (v: Vec<Func<'a>>) -> Self {
        let m = v
            .iter()
            .enumerate()
            .map(|(i, f)| (f.signature.clone(), i))
            .collect();
        Self {
            v,
            m,
            e: HashSet::new()
        }
    }

    fn get_origin (&self, offset: usize) -> Option<usize> {
        self.v 
            .iter()
            .position(|func| func.range.contains(&offset))

    }

    fn get_target<'b> (&self, siganture: Signature<'b>) -> Option<usize> {
        self.m.get(&siganture).map(|x| *x)
    }

    fn add_edge (&mut self, a: usize, b: usize) {
        self.e.insert((a,b));
    }
}

/// Populates Function.invocations by iterating over invocation tags.
/// TODO: Uses annotations to identify invoked function's membership.
fn make_graph<'a> (file: &'a str, funcs: Vec<Func<'a>>, tags: &Vec<(String, Tag)>) -> Graph<'a>
{
    let mut graph = Graph::from(funcs);

    for (_, invoke) in tags
        .iter()
        .filter(|(t, _)| *t == "call")
    {
        // Find originating function
        let origin = graph.get_origin(invoke.range.start)
            .expect(&format!(
                "could not find origin function for invocation: '{}'",
                &file[invoke.range.clone()]));
        
        // TODO: Classes
        let target = Signature { 
            name: &file[invoke.name_range.clone()],
            class: None };
        let target = graph.get_target(target);

        // It cannot find target, just continue.
        if target.is_none() {
            continue;
        }
        let target = target.unwrap();
        
        
        // Add invocation to function
        graph.add_edge(origin, target);
    }
    graph
}

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
}

#[derive(Serialize)]
struct Edge {
    key: usize, 
    source: usize,
    target: usize,
}

/// Convert into sigma.js's graph format, and serialize.
fn serialize<'a> (path: &'a str, graph: Graph) 
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
                    y: 214
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
