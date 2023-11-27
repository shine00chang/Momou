use tree_sitter::{Parser, QueryMatch, Query};

use super::*;

type Class<'a> = (Range<usize>, &'a str);

pub fn query (file: &str) -> (Vec<Func>, Vec<Invocation>){
    let mut parser = Parser::new();

    parser
        .set_language(tree_sitter_javascript::language())
        .expect("Error loading Rust grammar");

    let tree = parser.parse(file, None).unwrap();
    let root_node = tree.root_node();


    let classes = get_classes(&root_node, file);
    let funcs   = get_funcs(&root_node, file, &classes);
    let invocations = get_invocations(&root_node, file);

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


fn get_funcs<'a> (root_node: &tree_sitter::Node<'a>, file: &'a str, classes: &Vec<Class>) -> Vec<Func> 
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

    matches
        .map(|mat| {
            let range = get(&mat, "function").unwrap();
            let name  = get(&mat, "name").unwrap();
            let class = classes
                .iter()
                .find(|class| class.0.contains(&range.start))
                .map(|class| class.1.to_owned());

            Func {
                signature: Signature { 
                    name: file[name].to_owned(), 
                    class },
                range: range.clone(),
                snippet: file[range].to_owned()
            }
        })
        .collect()
}

fn get_invocations<'a> (root_node: &tree_sitter::Node<'a>, file: &'a str) -> Vec<Invocation> 
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
            let origin = get(&mat, "expr")
                .map(|expr| Range {
                    start: expr.start,
                    end: name.start -1,
                })
                .map(|origin| file[origin].to_owned());

            Invocation {
                name: file[name].to_owned(),
                origin,
                range,
            }
        })
        .collect()
}
