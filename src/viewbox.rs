use std::str::FromStr;

use anyhow::{Context as _, bail};

use crate::root::Root;

/// Represents the visible portion of an SVG canvas.
#[derive(Debug, Clone, Copy)]
pub struct ViewBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl FromStr for ViewBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        if parts.len() != 4 {
            bail!("invalid format");
        }

        Ok(Self {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        })
    }
}

impl ViewBox {
    /// Read the attributes on the root svg element to determine the `viewBox` size.
    pub fn from_root(root: &Root) -> Result<Self, anyhow::Error> {
        match root.attributes.get("viewBox") {
            Some(vb) => vb.parse(),
            None => Ok(Self {
                x: 0.0,
                y: 0.0,
                width: root
                    .attributes
                    .get("width")
                    .map_or("109.0", AsRef::as_ref)
                    .parse()
                    .context("invalid value for width")?,
                height: root
                    .attributes
                    .get("height")
                    .map_or("109.0", AsRef::as_ref)
                    .parse()
                    .context("invalid value for height")?,
            }),
        }
    }

    /// Horizontal coordinate (in user units) from the top-left corner.
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Vertical coordinate (in user units) from the top-left corner.
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Number of user units the `viewBox` spans vertically.
    pub const fn width(&self) -> f32 {
        self.width
    }

    /// Number of user units the `viewBox` spans horizontally.
    pub const fn height(&self) -> f32 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::root;

    use super::*;

    #[test]
    fn parse_viewbox() {
        let viewbox = "0 0 109 109"
            .parse::<ViewBox>()
            .expect("expected hardcoded viewBox to be valid");

        assert_eq!(viewbox.x, 0.0);
        assert_eq!(viewbox.y, 0.0);
        assert_eq!(viewbox.width, 109.0);
        assert_eq!(viewbox.height, 109.0);
    }

    #[test]
    fn has_viewbox() {
        let svg = r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="109" height="109" viewBox="0 0 109 109">
            </svg>
        "#;
        let reader = Cursor::new(svg);
        let root = root::from_reader(reader).expect("expected hardcoded svg to be valid");
        let viewbox = ViewBox::from_root(&root).expect("expected hardcoded viewbox to be valid");

        assert_eq!(viewbox.x(), 0.0);
        assert_eq!(viewbox.y(), 0.0);
        assert_eq!(viewbox.width(), 109.0);
        assert_eq!(viewbox.height(), 109.0);
    }

    #[test]
    fn no_viewbox() {
        let svg = r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="500" height="500">
            </svg>
        "#;
        let reader = Cursor::new(svg);
        let root = root::from_reader(reader).expect("expected hardcoded svg to be valid");
        let viewbox = ViewBox::from_root(&root).expect("expected hardcoded viewbox to be valid");

        assert_eq!(viewbox.x(), 0.0);
        assert_eq!(viewbox.y(), 0.0);
        assert_eq!(viewbox.width(), 500.0);
        assert_eq!(viewbox.height(), 500.0);
    }

    #[test]
    fn no_viewbox_or_wh() {
        let svg = r#"
            <svg xmlns="http://www.w3.org/2000/svg">
            </svg>
        "#;
        let reader = Cursor::new(svg);
        let root = root::from_reader(reader).expect("expected hardcoded svg to be valid");
        let viewbox = ViewBox::from_root(&root).expect("expected hardcoded viewbox to be valid");

        assert_eq!(viewbox.x(), 0.0);
        assert_eq!(viewbox.y(), 0.0);
        assert_eq!(viewbox.width(), 109.0);
        assert_eq!(viewbox.height(), 109.0);
    }
}
