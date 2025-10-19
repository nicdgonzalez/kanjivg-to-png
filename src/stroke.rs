use std::sync::LazyLock;

use anyhow::Context as _;
use regex::Regex;
use xmltree::Element;

use crate::viewbox::ViewBox;

static D_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[Mm]\s*([+-]?\d*\.?\d+)[, ]\s*([+-]?\d*\.?\d+)").unwrap());
static STROKE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"stroke\s*:\s*[^;]+;?").unwrap());
static DISPLAY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"display\s*:\s*none;?").unwrap());

pub fn hide_stroke(stroke: &mut Element) {
    let current = stroke.attributes.get("style").map_or("", AsRef::as_ref);
    let mut style = DISPLAY_RE.replace(current, "").to_string();
    style.push_str("display:none;");
    _ = stroke.attributes.insert("style".to_owned(), style);
}

pub fn unhide_stroke(stroke: &mut Element) {
    let current = stroke.attributes.get("style").map_or("", AsRef::as_ref);
    let mut style = DISPLAY_RE.replace(current, "").to_string();
    style.push_str("stroke:#000000;");
    _ = stroke.attributes.insert("style".to_owned(), style);
}

pub fn dim_stroke(stroke: &mut Element) {
    let current = stroke.attributes.get("style").map_or("", AsRef::as_ref);
    let mut style = STROKE_RE.replace(current, "").to_string();
    style.push_str("stroke:#999999;");
    _ = stroke.attributes.insert("style".to_owned(), style);
}

/// Create the circle that highlights where to start the stroke.
#[rustfmt::skip]
pub fn start_circle(
    stroke: &mut Element,
    viewbox: &ViewBox,
) -> Result<Element, anyhow::Error> {
    let d = stroke.attributes.get("d").map_or("", AsRef::as_ref);
    let Some(captures) = D_RE.captures(d) else {
        anyhow::bail!("invalid value for 'd' attribute");
    };
    let cx = captures.get(1).context("expected cx to have a value")?;
    let cy = captures.get(2).context("expected cy to have a value")?;
    let radius = viewbox.width().min(viewbox.height()) * 0.04;

    let mut circle = Element::new("circle");
    circle.attributes.insert("cx".to_owned(), cx.as_str().to_owned());
    circle.attributes.insert("cy".to_owned(), cy.as_str().to_owned());
    circle.attributes.insert("r".to_owned(), radius.to_string());
    circle.attributes.insert("fill".to_owned(), "#ff0000".to_owned());
    circle.attributes.insert("fill-opacity".to_owned(), 0.6.to_string());
    circle.attributes.insert("stroke".to_owned(), "none".to_owned());
    circle.attributes.insert("id".to_owned(), "kanji_dot".to_owned());
    Ok(circle)
}
