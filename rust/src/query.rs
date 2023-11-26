use tree_sitter_tags::{TagsContext, TagsConfiguration, Tag};

use super::*;

pub fn query (file: &str) -> (Vec<Func>, Vec<Invocation>){
    let tags = get_tags(&file);
    let funcs = make_funcs(&file, &tags);
    let invocations = make_invocations(&file, &tags);

    (funcs, invocations)
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
