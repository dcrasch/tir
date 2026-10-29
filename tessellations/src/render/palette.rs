//! Colour palettes for tilings, shared by the renderers.
//!
//! Renderers colour a tile with entry `colour % figure.colors` of the palette,
//! so the order matters: the first entries are used by every figure, the later
//! ones only by figures with more colours.

use palette::Srgb;

/// A named set of tile colours, in colour-index order.
pub struct Palette {
    /// Display name, e.g. `"Alhambra"`.
    pub name: &'static str,
    /// sRGB bytes for each colour.
    pub colors: &'static [[u8; 3]],
}

impl Palette {
    /// Number of colours in the palette.
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// True if the palette has no colours.
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Colours as CSS hex strings, e.g. `"#c96c9b"`.
    pub fn css(&self) -> Vec<String> {
        self.colors
            .iter()
            .map(|&[r, g, b]| format!("#{r:02x}{g:02x}{b:02x}"))
            .collect()
    }

    /// Colours as `Srgb` floats in the range 0..=1.
    pub fn srgb(&self) -> Vec<Srgb<f32>> {
        self.colors
            .iter()
            .map(|&[r, g, b]| Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
            .collect()
    }
}

/// Four ceramic colours sampled from the Alhambra "Brilliant" palette:
/// Amalfi, Capri, Canary and Denim.
///
/// Two-colour figures get rose and teal, three-colour figures add gold, and
/// four-colour figures add the deep blue.
pub const ALHAMBRA: Palette = Palette {
    name: "Alhambra",
    colors: &[
        [0xc9, 0x6c, 0x9b], // Amalfi
        [0x56, 0x87, 0x83], // Capri
        [0xdb, 0xc2, 0x4b], // Canary
        [0x1b, 0x1c, 0x47], // Denim
    ],
};

/// The classic red, green, blue and black colours of the early SVGs.
pub const CLASSIC: Palette = Palette {
    name: "Classic",
    colors: &[
        [0xff, 0x00, 0x00],
        [0x00, 0x80, 0x00],
        [0x00, 0x00, 0xff],
        [0x00, 0x00, 0x00],
    ],
};

/// The Alhambra colours with their saturation removed, for printing.
pub const GRAYSCALE: Palette = Palette {
    name: "Grayscale",
    colors: &[
        [0xdd, 0xdd, 0xdd],
        [0x99, 0x99, 0x99],
        [0x55, 0x55, 0x55],
        [0x11, 0x11, 0x11],
    ],
};

/// All palettes; [`DEFAULT_PALETTE`] is the first one.
pub const PALETTES: &[Palette] = &[ALHAMBRA, CLASSIC, GRAYSCALE];

/// The palette renderers start with.
pub const DEFAULT_PALETTE: Palette = ALHAMBRA;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_the_first_palette() {
        assert_eq!(DEFAULT_PALETTE.name, PALETTES[0].name);
        assert!(!PALETTES.is_empty());
    }

    #[test]
    fn every_palette_colours_four_tiles() {
        for palette in PALETTES {
            assert_eq!(palette.len(), 4, "{}", palette.name);
            assert!(!palette.is_empty());
        }
    }

    #[test]
    fn palettes_have_no_duplicate_colours() {
        for palette in PALETTES {
            for (i, a) in palette.colors.iter().enumerate() {
                for (j, b) in palette.colors.iter().enumerate().take(i) {
                    assert_ne!(a, b, "{} colours {j} and {i} are identical", palette.name);
                }
            }
        }
    }

    #[test]
    fn css_is_lowercase_hex() {
        assert_eq!(ALHAMBRA.css()[0], "#c96c9b");
        assert_eq!(CLASSIC.css()[3], "#000000");
        assert_eq!(GRAYSCALE.css()[0], "#dddddd");
    }

    #[test]
    fn srgb_is_normalised() {
        let colors = ALHAMBRA.srgb();
        assert_eq!(colors.len(), 4);
        assert!((colors[0].red - 0xc9 as f32 / 255.0).abs() < f32::EPSILON);
        assert!((colors[3].blue - 0x47 as f32 / 255.0).abs() < f32::EPSILON);
    }
}
