use rayon::prelude::*;
use std::convert::AsRef;
use std::path::Path;
use clap::Parser;

pub mod json;
use crate::json::Json;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const VERSION: &str = "1";

#[derive(Parser)]
struct CommandLine {
    /// Output path for produced JSON
    #[arg(short,long)]
    output: String,

    /// Generate JSON from cppreference dataset
    #[arg(short,long)]
    cppreference: Option<String>
}

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

fn extract_links_from_html_file<P: AsRef<Path> + std::fmt::Debug + Clone>(path: P) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path.clone())?;
    let html = tl::parse(&content, tl::ParserOptions::default())?;

    Ok(html
        .query_selector("a[href]")
        .expect("Query selector is well written, I promise")
        .filter_map(|element| extract_attribute(html.parser(), element, "href"))
        .filter(|href| {
            !href.contains("http:") && !href.contains("https:") && !href.contains("ftp:")
        })
        .map(|uri| match uri.find('#') {
            Some(start_of_fragment) => &uri[..start_of_fragment],
            None => uri,
        })
        .flat_map(urlencoding::decode)
        .map(String::from)
        .collect())
}

pub fn generate_cpp_dataset<Out: std::io::Write>(cppreference: &str, out: Out) -> Result<()>
{
    let paths = glob::glob(cppreference)
        .expect("Valid glob pattern")
        .flatten()
        .filter(|path| path.is_file())
        .flat_map(std::fs::canonicalize)
        .collect::<Vec<_>>();

    let links_for_pages: Vec<_> = paths
        .par_iter()
        .map(|path| (path, extract_links_from_html_file(path).unwrap()))
        .collect();

    let mut json = json::with_output(out);
    let mut obj = json.object()?;
    obj.key("version")?;
    obj.set(VERSION)?;

    obj.key("pages")?;
    let mut pages = obj.object()?;


    for (page, links) in links_for_pages {
        pages.key(page.to_str().unwrap())?;
        let mut current_page = pages.array()?;

        for link in links {
            let mut full_link = page.parent().unwrap().to_owned();
            full_link.push(link.clone());
            if !full_link.exists() {
                // This is a fix for broken links that are result error within original data.
                // We decide to not investigate it further and accept it.
                if full_link.as_os_str().to_string_lossy().contains("ranges") {
                    continue;
                }
                // For this dataset this should be unreachable
                unreachable!();
            }
            let full_link = full_link.canonicalize()?;
            current_page.set(full_link.to_str().unwrap())?;
        }
    }

    Ok(())
}

fn main() {
    let args = CommandLine::parse();

    if let Some(cppreference) = args.cppreference {
        let out = std::io::BufWriter::new(std::fs::File::create(args.output).expect("Open file"));
        generate_cpp_dataset(&(cppreference + "/**/*.html"), out).expect("Dataset generation");
    }
}
