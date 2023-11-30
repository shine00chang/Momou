
use super::*;

impl Graph {
    fn from (v: Functions) -> Self {
        let v = v.v;
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

    fn get_target (&self, siganture: &Signature) -> Option<usize> {
        self.m.get(&siganture).map(|x| *x)
    }

    fn add_edge (&mut self, a: usize, b: usize) {
        self.e.insert((a,b));
    }
}

/// Populates Function.invocations by iterating over invocation tags.
/// TODO: Uses annotations to identify invoked function's membership.
pub fn make<'a> (file: &'a str, funcs: Functions, invocations: &Vec<Invocation>) -> Graph
{
    let mut graph = Graph::from(funcs);

    for x in &graph.m {
        println!("-{:?}", x);
    }
    for x in invocations {
        print!("{:?}", x);
        if graph.get_target(&x.signature).is_some() {
            println!("   {BLD} Eureka! {RST}");
        } else {
            println!();
        }
    }

    for invoke in invocations {
        // Find originating function
        let origin = graph.get_origin(invoke.range.start);

        // If no origin, then is global invocation.
        if origin.is_none() {
            println!("-> {BLD}Note:{RST} Global invocation found at position: {}", invoke.range.start);
            continue;
        }
        let origin = origin.unwrap();
        
        // TODO: Classes
        let target = &invoke.signature; 
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
