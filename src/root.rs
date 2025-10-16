use std::fs;
use std::io::{BufRead, Cursor};
use std::path::Path;
use std::sync::LazyLock;

use anyhow::{Context as _, bail};
use regex::Regex;
use xmltree::{Element, XMLNode};

pub type Root = Element;
pub type NodePath = Vec<usize>;

static STROKE_NUMBERS_GROUP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^kvg:StrokeNumbers_0[a-f0-9]{4}$").unwrap());

/// Read an SVG from the file at `path`.
pub fn from_file<P>(path: P) -> Result<Root, anyhow::Error>
where
    P: AsRef<Path>,
{
    let text = fs::read_to_string(path).context("failed to read input file")?;

    // KanjiVG defines the `kvg` namespace within the DTD section (the `<!DOCTYPE>` section),
    // which is valid XML, but the library we're using doesn't process the DTD section.
    let text = if text.contains("xmlns:kvg=") {
        text
    } else {
        text.replacen("<svg ", "<svg xmlns:kvg=\"http://kanjivg.tagaini.net\" ", 1)
    };

    let reader = Cursor::new(text);
    from_reader(reader)
}

/// Read an SVG from `reader`.
pub fn from_reader<R>(reader: R) -> Result<Root, anyhow::Error>
where
    R: BufRead,
{
    Element::parse(reader).context("failed to parse SVG")
}

/// Get the path to each stroke.
pub fn get_strokes(root: &Root) -> Vec<NodePath> {
    fn walk(
        nodes: &[XMLNode],
        stroke_path_re: &Regex,
        node_path: &mut NodePath,
        strokes: &mut Vec<NodePath>,
    ) {
        for (i, node) in nodes.iter().enumerate() {
            node_path.push(i);

            if let XMLNode::Element(element) = node {
                match element.name.as_ref() {
                    "g" => walk(&element.children, stroke_path_re, node_path, strokes),
                    "path" if is_stroke(element, stroke_path_re) => strokes.push(node_path.clone()),
                    _ => {}
                }
            }

            node_path.pop();
        }
    }

    let mut strokes = Vec::new();
    let mut path = NodePath::new();
    let stroke_path_re = Regex::new(r"^kvg:0[a-f0-9]{4}-s\d+$").unwrap();
    walk(&root.children, &stroke_path_re, &mut path, &mut strokes);
    strokes
}

/// Check if the element's ID matches the format for strokes.
fn is_stroke(element: &Element, re: &Regex) -> bool {
    element
        .attributes
        .get("id")
        .is_some_and(|id| re.is_match(id))
}

/// Remove all of the text stroke numbers.
pub fn delete_stroke_numbers_group(root: &mut Root) {
    root.children.retain(|node| !is_stroke_numbers_group(node));
}

fn is_stroke_numbers_group(node: &XMLNode) -> bool {
    node.as_element()
        .and_then(|e| e.attributes.get("id"))
        .is_some_and(|id| STROKE_NUMBERS_GROUP_RE.is_match(id))
}

pub fn get_stroke<'a>(
    root: &'a mut Root,
    path: &[usize],
) -> Result<&'a mut Element, anyhow::Error> {
    let mut children: &'a mut [XMLNode] = &mut root.children;

    for (depth, &i) in path.iter().enumerate() {
        if i >= children.len() {
            bail!("target path is out of bounds");
        }

        if depth == path.len() - 1 {
            if let XMLNode::Element(ref mut element) = children[i] {
                return Ok(element);
            }

            bail!("expected Element node");
        }

        match children[i] {
            XMLNode::Element(ref mut element) => {
                children = &mut element.children;
            }
            _ => bail!("expected Element node"),
        }
    }

    bail!("empty node path");
}
