// The code is in a weird middle stage where it started out being optimized for my specific
// use case, but is in the process of being refactored to make it easier for others to edit it...

#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

mod background;
mod root;
mod stroke;
mod viewbox;

use std::io::{self, Write as _};
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context as _, bail};
use clap::Parser as _;
use colored::Colorize;

use image::{GenericImage as _, ImageBuffer, RgbaImage};
use tiny_skia::Pixmap;
use usvg::{Transform, Tree};
use viewbox::ViewBox;
use xmltree::{Element, XMLNode};

use crate::background::background;
use crate::root::{NodePath, Root, delete_stroke_numbers_group, get_strokes};
use crate::stroke::{dim_stroke, hide_stroke, start_circle, unhide_stroke};

#[derive(Debug, clap::Parser)]
#[clap(
    about = "Convert a KanjiVG SVG into a PNG that shows how the Kanji is drawn, stroke by stroke.",
    after_help = format!(
        "{}: https://github.com/nicdgonzalez/kanjivg-to-png",
        "Repository".bold(),
    ),
    version,
)]
struct Parser {
    #[arg(short, long, help = "Path to the KanjiVG SVG file")]
    input: PathBuf,

    #[arg(short, long, help = "Path to save the generated PNG file")]
    output: PathBuf,
}

/// The main entry point to the program.
///
/// This function executes the main program and reports any error messages to the user.
fn main() -> ExitCode {
    try_main().unwrap_or_else(|err| {
        let mut stderr = io::stderr().lock();
        _ = writeln!(stderr, "{}", "kanjivg-to-png failed".bold().red());

        for cause in err.chain() {
            _ = writeln!(stderr, "  {}: {}", "Cause".bold(), cause);
        }

        ExitCode::FAILURE
    })
}

/// Edits the SVG and creates the final combined PNG.
fn try_main() -> Result<ExitCode, anyhow::Error> {
    let args = Parser::parse();
    let mut root = root::from_file(&args.input)?;
    let viewbox = ViewBox::from_root(&root)?;

    // Insert at 0 so it appears behind everything else.
    root.children
        .insert(0, XMLNode::Element(background(&viewbox)));

    delete_stroke_numbers_group(&mut root);

    let strokes = get_strokes(&root);

    if strokes.is_empty() {
        bail!("expected Kanji to have at least one stroke");
    }

    // Hide all except for the first stroke.
    for path in strokes.iter().skip(1) {
        let stroke = root::get_stroke(&mut root, path).context("failed to get stroke")?;
        hide_stroke(stroke);
    }

    let mut frames = Vec::new();

    // Since the first stroke doesn't have a previous, we handle it separately.
    prepare_stroke(&mut root, &strokes[0], &viewbox)?;
    let frame = create_frame(&root).context("failed to create stroke frame")?;
    frames.push(frame);

    // Create frames for the remaining strokes.
    for i in 1..strokes.len() {
        let previous = root::get_stroke(&mut root, &strokes[i - 1])
            .context("failed to get previous stroke")?;
        dim_stroke(previous);

        prepare_stroke(&mut root, &strokes[i], &viewbox)?;
        let frame = create_frame(&root).context("failed to create frame")?;
        frames.push(frame);
    }

    let image = combine_frames(&frames)?;
    image.save(&args.output)?;

    Ok(ExitCode::SUCCESS)
}

fn prepare_stroke(root: &mut Root, path: &[usize], viewbox: &ViewBox) -> Result<(), anyhow::Error> {
    let stroke = root::get_stroke(root, path)?;
    unhide_stroke(stroke);

    // Update the start circle.
    let start_circle = start_circle(stroke, viewbox)?;
    root.children.retain(|n| !is_start_circle(n));
    root.children.push(XMLNode::Element(start_circle));

    Ok(())
}

fn is_start_circle(node: &XMLNode) -> bool {
    node.as_element()
        .and_then(|e| e.attributes.get("id"))
        .is_some_and(|id| id == "kanji_dot")
}

fn create_frame(root: &Element) -> Result<RgbaImage, anyhow::Error> {
    let mut data = Vec::new();
    root.write(&mut data)?;

    let tree_opts = usvg::Options::default();
    let tree = Tree::from_data(&data, &tree_opts).context("failed to parse SVG")?;

    let scale = 2.0;

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let width = (tree.size().width() * scale).round() as u32;

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let height = (tree.size().height() * scale) as u32;

    let mut pixmap = Pixmap::new(width, height).context("failed to create pixmap")?;
    let transform = Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let image = RgbaImage::from_raw(width, height, pixmap.data().to_vec())
        .context("failed to build image buffer")?;

    Ok(image)
}

fn combine_frames(frames: &[RgbaImage]) -> Result<RgbaImage, anyhow::Error> {
    let width = frames.iter().map(ImageBuffer::width).sum::<u32>();
    let height = frames
        .iter()
        .map(ImageBuffer::height)
        .max()
        .context("no frames available")?;

    let mut image = RgbaImage::new(width, height);
    let mut x_offset = 0;

    for frame in frames {
        image
            .copy_from(frame, x_offset, 0)
            .context("failed to copy frame to final PNG")?;
        x_offset += frame.width();
    }

    Ok(image)
}
