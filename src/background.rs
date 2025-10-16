use xmltree::{Element, XMLNode};

use crate::viewbox::ViewBox;

/// Background that will render behind each frame in the diagram.
pub fn background(viewbox: &ViewBox) -> Element {
    let mut group = Element::new("g");

    let background = create_rect(
        &viewbox.x(),
        &viewbox.y(),
        &viewbox.width(),
        &viewbox.height(),
        &"#ffffff",
        &"none",
        &0,
    );

    let border = create_rect(
        &(viewbox.x() + 0.5),
        &(viewbox.y() + 0.5),
        &(viewbox.width() - 1.0).max(0.0),
        &(viewbox.height() - 1.0).max(0.0),
        &"none",
        &"#cccccc",
        &1,
    );

    let cx = viewbox.x() + viewbox.width() / 2.0;
    let cy = viewbox.y() + viewbox.height() / 2.0;
    let dash = "4,4";

    // Center grid lines that help to position the Kanji.
    let horizontal = create_line(
        &viewbox.x(),
        &cy,
        &(viewbox.x() + viewbox.width()),
        &cy,
        &"#cccccc",
        &1,
        &dash,
    );
    let vertical = create_line(
        &cx,
        &viewbox.y(),
        &cx,
        &(viewbox.y() + viewbox.height()),
        &"#cccccc",
        &1,
        &dash,
    );

    group.children.extend([
        XMLNode::Element(background),
        XMLNode::Element(horizontal),
        XMLNode::Element(vertical),
        XMLNode::Element(border),
    ]);

    group
}

/// A helper function for creating `rect` elements.
#[rustfmt::skip]
fn create_rect(
    x: &impl ToString,
    y: &impl ToString,
    width: &impl ToString,
    height: &impl ToString,
    fill: &impl ToString,
    stroke: &impl ToString,
    stroke_width: &impl ToString,
) -> Element {
    let mut rect = Element::new("rect");
    rect.attributes.insert("x".to_owned(), x.to_string());
    rect.attributes.insert("y".to_owned(), y.to_string());
    rect.attributes.insert("width".to_owned(), width.to_string());
    rect.attributes.insert("height".to_owned(), height.to_string());
    rect.attributes.insert("fill".to_owned(), fill.to_string());
    rect.attributes.insert("stroke".to_owned(), stroke.to_string());
    rect.attributes.insert("stroke-width".to_owned(), stroke_width.to_string());
    rect
}

/// A helper function for creating `line` elements.
#[rustfmt::skip]
fn create_line(
    x1: &impl ToString,
    y1: &impl ToString,
    x2: &impl ToString,
    y2: &impl ToString,
    stroke: &impl ToString,
    stroke_width: &impl ToString,
    dash: &impl ToString,
) -> Element {
    let mut line = Element::new("line");
    line.attributes.insert("x1".to_owned(), x1.to_string());
    line.attributes.insert("y1".to_owned(), y1.to_string());
    line.attributes.insert("x2".to_owned(), x2.to_string());
    line.attributes.insert("y2".to_owned(), y2.to_string());
    line.attributes.insert("stroke".to_owned(), stroke.to_string());
    line.attributes.insert("stroke-width".to_owned(), stroke_width.to_string());
    line.attributes.insert("stroke-dasharray".to_owned(), dash.to_string());
    line
}
