use tree_sitter::{Parser, Language, QueryMatch, Query};
use tree_sitter_tags::{TagsContext, TagsConfiguration, Tag};

use super::*;

pub fn query (file: &str) -> (Vec<Func>, Vec<Invocation>){
    let mut parser = Parser::new();

    parser.set_language(tree_sitter_javascript::language())
        .expect("Error loading Rust grammar");

    let tree = parser.parse(file, None).unwrap();
    let root_node = tree.root_node();

    const TAG_QUERY_FILE: &str = "functions.scm";
    let query_file = std::fs::read_to_string(&TAG_QUERY_FILE)
        .expect(&format!("{BLD}failed to open query file '{}'{RST}", &TAG_QUERY_FILE));

    let query = tree_sitter::Query::new(
        tree_sitter_javascript::language(),
        &query_file)
        .unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let matches = cursor.matches(
        &query,
        root_node,
        file.as_bytes());

    let mut funcs = vec![];

    
    let get = get_maker(&query);


    let classes = get_classes(&root_node, file);
    println!("{:?}", classes);
    for mat in matches {
        // Switch for query type
        if get(&mat, "function").is_some() {
            println!(" - !! is function !!");
            let range = get(&mat, "function").unwrap();
            let name  = get(&mat, "name").unwrap();
            let class = classes
                .iter()
                .find(|class| class.0.contains(&range.start))
                .map(|class| class.1.to_owned());

            let func = Func {
                signature: Signature { 
                    name: file[name].to_owned(), 
                    class },
                range,
            };
            funcs.push(func);
        }
    }

    (funcs, vec![])
}

fn get_maker<'a> (query: &'a Query) -> impl for<'b> Fn(&'b QueryMatch, &'static str) -> Option<Range<usize>> + 'a {
    |mat: &QueryMatch, capture_name| {
        let i = query.capture_index_for_name(capture_name).unwrap();
        mat.captures.iter().find(|cap| cap.index == i).map(|cap| cap.node.byte_range().clone())
    }
}

fn get_classes<'a> (root_node: &tree_sitter::Node<'a>, file: &'a str) -> Vec<(Range<usize>, &'a str)> 
{
    const TAG_QUERY_FILE: &str = "classes.scm";
    let query_file = std::fs::read_to_string(&TAG_QUERY_FILE)
        .expect(&format!("{BLD}failed to open query file '{}'{RST}", &TAG_QUERY_FILE));

    let query = tree_sitter::Query::new(
        tree_sitter_javascript::language(),
        &query_file)
        .unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let matches = cursor.matches(
        &query,
        *root_node,
        file.as_bytes());

    let get = get_maker(&query);

    matches
        .filter_map(|mat| 
            get(&mat, "class")
                .map(|range| (range, &file[get(&mat, "name").unwrap()]))
        )
        .collect()
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
fn make_funcs<'a> (file: &'a str, tags: &'a Vec<(String, Tag)>) -> Vec<Func> 
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
                .map(|class| file[class.name_range.clone()].to_owned());
            Func {
                signature: Signature { 
                    name: file[func.name_range.clone()].to_owned(), 
                    class },
                range: func.range.clone(),
            }
        })
        .collect()
}


fn make_invocations<'a> (file: &'a str, tags: &'a Vec<(String, Tag)>) -> Vec<Invocation> {
    tags.iter()
        .filter(|(t, _)| *t == "call")
        .map(|(_, invoke)| {
            // Find originating function
            Invocation { 
                name: file[invoke.name_range.clone()].to_owned(),
                range: invoke.range.clone()
            }
        })
        .collect()
}
