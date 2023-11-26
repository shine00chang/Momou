mod query;
mod graph;
mod serializer;


use std::collections::{HashSet, HashMap};
use std::env;
use std::fs;
use std::ops::Range;
use std::path::Path;

pub const RST: &str = "\x1b[0m";
pub const BLD: &str = "\x1b[1m";
pub const HLT: &str = "\x1b[48;5;226m";


#[derive(Debug, Clone, Default)]
pub struct Func {
    signature: Signature, 
    range: Range<usize>,
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct Signature {
    name: String,
    class: Option<String>
}

pub struct Invocation {
    name: String,
    range: Range<usize>
}

impl Signature {
    fn to_string (&self) -> String {
        if let Some(class) = &self.class {
            let mut s = class.clone();
            s.push_str("::");
            s.push_str(&self.name);
            s 
        } else {
            self.name.clone()
        }
    }
}


pub struct Graph {
    v: Vec<Func>,
    m: HashMap<Signature, usize>,
    e: HashSet<(usize, usize)>
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

    // let funcs = find_side_effects(&file, funcs, &tags);
    let (funcs, invocations) = query::query(&file);

    let graph = graph::make(&file, funcs, &invocations);
    serializer::serialize(output_path, graph);
}


