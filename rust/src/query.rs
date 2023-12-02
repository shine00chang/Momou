use tree_sitter::{Parser, QueryMatch, Query};

use super::*;

type Class<'a> = (Range<usize>, &'a str);

#[derive(Default)]
struct Annotations<'a> {
    m: HashMap<&'a str, Vec<(usize, &'a str)>>,
    // Internal immutability guarantee. (questionable design pattern)
    lock: bool,
}
impl<'a> Annotations<'a> {
    fn insert (&mut self, k: &'a str, v: (usize, &'a str)) {
        assert!(!self.lock);
        if let Some(vec) = self.m.get_mut(k) {
            vec.push(v);
        } else {
            self.m.insert(k, vec![v]);
        }
    }

    fn lock (&mut self) {
        self.lock = true;
    }

    fn get (&self, k: &'a str, range: &Range<usize>) -> Option<&'a str> {
        assert!(self.lock);
        println!("{} => {:?}, {:?}", k, self.m.get(k), range);
        self.m.get(k).and_then(|v|
            v.iter()
                .rev()
                .find(|&(pos, _)| range.contains(pos))
                .map(|&(_, v)| v)
            )
    }
}

impl Functions {
    fn from (v: Vec<Func>) -> Self {
        Self { v }
    }
    fn get_origin (&self, offset: usize) -> Option<&Func> {
        self.v 
            .iter()
            .find(|func| func.range.contains(&offset))

    }
}

pub fn query (file: &str) -> (Functions, Vec<Invocation>){
    let mut parser = Parser::new();

    parser
        .set_language(tree_sitter_javascript::language())
        .expect("Error loading Rust grammar");

    let tree = parser.parse(file, None).unwrap();
    let root_node = tree.root_node();


    let classes = get_classes(&root_node, file);
    let funcs   = get_funcs(&root_node, file, &classes);
    let annotations = get_annotations(&root_node, file);
    let invocations = get_invocations(&root_node, file, &funcs, &annotations);

    (funcs, invocations)
}

fn get_maker<'a> (query: &'a Query) -> impl for<'b> Fn(&'b QueryMatch, &'static str) -> Option<Range<usize>> + 'a {
    |mat: &QueryMatch, capture_name| {
        let i = query.capture_index_for_name(capture_name)
            .expect(&format!("none such capture named '{}'", capture_name));
        mat.captures.iter().find(|cap| cap.index == i).map(|cap| cap.node.byte_range().clone())
    }
}

fn get_classes<'a> (root_node: &tree_sitter::Node<'a>, file: &'a str) -> Vec<Class<'a>> 
{
    const TAG_QUERY_FILE: &str = "queries/classes.scm";
    let query_file = std::fs::read_to_string(&TAG_QUERY_FILE)
        .expect(&format!("{BLD}failed to open query file '{}'{RST}", &TAG_QUERY_FILE));

    let query = tree_sitter::Query::new(
        tree_sitter_javascript::language(),
        &query_file)
        .unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let matches = cursor.matches(&query, *root_node, file.as_bytes());

    let get = get_maker(&query);

    matches
        .filter_map(|mat| get(&mat, "class")
            .map(|range| (range, &file[get(&mat, "name").unwrap()])))
        .collect()
}


fn get_funcs<'a> (root_node: &tree_sitter::Node<'a>, file: &'a str, classes: &Vec<Class>) -> Functions
{
    const TAG_QUERY_FILE: &str = "queries/functions.scm";
    let query_file = std::fs::read_to_string(&TAG_QUERY_FILE)
        .expect(&format!("{BLD}failed to open query file '{}'{RST}", &TAG_QUERY_FILE));

    let query = tree_sitter::Query::new(
        tree_sitter_javascript::language(),
        &query_file)
        .unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let matches = cursor.matches(&query, *root_node, file.as_bytes());
    let get = get_maker(&query);

    let v: Vec<_> = matches
        .map(|mat| {
            let range = get(&mat, "function").unwrap();
            let name  = get(&mat, "name").unwrap();
            let class = classes
                .iter()
                .find(|class| class.0.contains(&range.start))
                .map(|class| class.1.to_owned());
            let snippet = Range { 
                start: range.start,
                end: range.end.min(range.start + 200)
            };

            Func {
                signature: Signature { 
                    name: file[name].to_owned(), 
                    class },
                range: range.clone(),
                value: (range.end - range.start).min(50),
                snippet: file[snippet].to_owned()
            }
        })
        .collect();

    Functions::from(v)
}

fn get_invocations<'a> (
    root_node: &tree_sitter::Node<'a>,
    file: &'a str,
    functions: &Functions,
    annotations: &Annotations) -> Vec<Invocation> 
{
    const TAG_QUERY_FILE: &str = "queries/invocations.scm";
    let query_file = std::fs::read_to_string(&TAG_QUERY_FILE)
        .expect(&format!("{BLD}failed to open query file '{}'{RST}", &TAG_QUERY_FILE));

    let query = tree_sitter::Query::new(
        tree_sitter_javascript::language(),
        &query_file)
        .unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let matches = cursor.matches(&query, *root_node, file.as_bytes());
    let get = get_maker(&query);

    matches
        .map(|mat| {
            let range = get(&mat, "invocation").unwrap();
            let name  = get(&mat, "name").unwrap();
            let object = get(&mat, "expr")
                .map(|expr| Range {
                    start: expr.start,
                    end: name.start -1,
                })
                .map(|obj| file[obj].to_owned());
            let name = &file[name];
            let origin = functions.get_origin(range.start)
                // Defaults to global scope (everything)
                .map_or_else(|| 0..file.len(), |f| f.range.clone());

            if let Some(name) = &object {
                println!("- Invocation with object: '{BLD}{}{RST}'", name);
            }

            let signature = Signature {
                name: name.to_owned(),
                class: object.and_then(|obj| 
                    annotations
                        .get(&obj[..], &origin)
                        .map(|s| s.to_owned())
                    )
            };

            Invocation {
                signature,
                range,
            }
        })
        .collect()
}

fn get_annotations<'a> (root_node: &tree_sitter::Node<'a>, file: &'a str) -> Annotations<'a>
{
    const TAG_QUERY_FILE: &str = "queries/annotations.scm";
    let query_file = std::fs::read_to_string(&TAG_QUERY_FILE)
        .expect(&format!("{BLD}failed to open query file '{}'{RST}", &TAG_QUERY_FILE));

    let query = tree_sitter::Query::new(
        tree_sitter_javascript::language(),
        &query_file)
        .unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let matches = cursor.matches(&query, *root_node, file.as_bytes());
    let get = get_maker(&query);

    let mut m: Annotations = Default::default();

    for mat in matches {
        let name = get(&mat, "name").unwrap();
        let name = &file[name];
        let comment = get(&mat, "annotation").unwrap();
        let start = comment.start;
        let comment = &file[comment];

        let a = comment.find(":")
            .expect("could not find ':' in class annotation. Check query Regex.");
        let t = &comment[a+1..];
        let b = t.find(|c| !char::is_ascii_alphanumeric(&c) && c != '_').unwrap_or(t.len());
        let class = &t[..b];
        
        println!("-> {BLD}Note{RST}: Annotation found for '{}' as class '{}'", name, class);

        m.insert(name, (start, class))
    }

    // Internal immutability guarantee. (questionable design pattern)
    m.lock();
    m
}
