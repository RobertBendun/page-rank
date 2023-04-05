use std::convert::AsRef;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn extract_attribute<'a>(
    parser: &'a tl::Parser,
    node: tl::NodeHandle,
    key: &'a str,
) -> Option<&'a str> {
    node.get(parser)?
        .as_tag()?
        .attributes()
        .get(key)
        .flatten()?
        .try_as_utf8_str()
}

fn extract_links_from_html_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    let html = tl::parse(&content, tl::ParserOptions::default())?;

    Ok(html
        .query_selector("a[href]")
        .expect("Query selector is well written, I promise")
        .filter_map(|element| extract_attribute(html.parser(), element, "href"))
        //.filter(|href| !href.starts_with("http://") && !href.starts_with("https://"))
        .map(|s| String::from(s))
        .collect())
}

fn main() {
    let mut paths_count = 0;
    for path in glob::glob("../reference/**/*.html").expect("Valid glob pattern").flatten() {
        match extract_links_from_html_file(path.clone()) {
            Ok(vec) => paths_count += vec.len(),
            Err(error) => println!("Path {path:?} failed: {error}"),
        }
    }
    println!("Count: {paths_count}");
}
