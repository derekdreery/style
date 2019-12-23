use std::fmt;

/// A color that possibly is possibly code, rather than a literal
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicColor {
    Literal(Color),
    /// The type of the block is not checked here (it is checked by typeck).
    Dynamic(syn::Block),
}

impl DynamicColor {
    pub fn is_dynamic(&self) -> bool {
        match self {
            DynamicColor::Dynamic(_) => true,
            DynamicColor::Literal(_) => false,
        }
    }
}

impl fmt::Display for DynamicColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DynamicColor::Dynamic(_) => Ok(()),
            DynamicColor::Literal(color) => color.fmt(f),
        }
    }
}

// TODO other color variants.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Color {
    HexRGB(u8, u8, u8),
    HexRGBA(u8, u8, u8, u8),
    // Invariants: `0 <= .0 < 360`, `0 <= .1 < 100`, `0 <= .2 < 100`.
    HSL(f64, f64, f64),
    // Invariants: `0 <= .0 < 360`, `0 <= .1 < 100`, `0 <= .2 < 100`, `0 <= .3 < 1`.
    HSLA(f64, f64, f64, f64),
}

impl Color {
    // todo similar for others
    pub fn to_rgb(self) -> Color {
        use Color::*;
        match self {
            HexRGB(r, g, b) => HexRGB(r, g, b),
            HexRGBA(r, g, b, _) => HexRGB(r, g, b),
            HSL(h, s, l) => {
                let s = s * 0.01;
                let l = l * 0.01;
                let (r, g, b) = hsl_to_rgb(h, s, l);
                HexRGB((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            }
            HSLA(h, s, l, _) => Color::to_rgb(HSL(h, s, l)),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::HexRGB(r, g, b) => write!(f, "#{:02x}{:02x}{:02x}", r, g, b),
            Color::HexRGBA(r, g, b, a) => write!(f, "#{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
            Color::HSL(h, s, l) => write!(f, "hsl({}, {}%, {}%)", h, s, l),
            Color::HSLA(h, s, l, a) => write!(f, "hsla({}, {}%, {}%, {})", h, s, l, a),
        }
    }
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    debug_assert!(h >= 0.0 && h < 360.0);
    debug_assert!(s >= 0.0 && s <= 1.0);
    debug_assert!(l >= 0.0 && l <= 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c * 0.5;
    let (rp, gp, bp) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (rp + m, gp + m, bp + m)
}

pub fn parse_hex(hex: &str) -> Option<Color> {
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(hex.get(0..1)?, 16).ok()?;
            let g = u8::from_str_radix(hex.get(1..2)?, 16).ok()?;
            let b = u8::from_str_radix(hex.get(2..3)?, 16).ok()?;
            // #fff is equivalent to #ffffff
            Some(Color::HexRGB(r << 4 | r, g << 4 | g, b << 4 | b))
        }
        6 => {
            let r = u8::from_str_radix(hex.get(0..2)?, 16).ok()?;
            let g = u8::from_str_radix(hex.get(2..4)?, 16).ok()?;
            let b = u8::from_str_radix(hex.get(4..6)?, 16).ok()?;
            Some(Color::HexRGB(r, g, b))
        }
        8 => {
            let r = u8::from_str_radix(hex.get(0..2)?, 16).ok()?;
            let g = u8::from_str_radix(hex.get(2..4)?, 16).ok()?;
            let b = u8::from_str_radix(hex.get(4..6)?, 16).ok()?;
            let a = u8::from_str_radix(hex.get(6..8)?, 16).ok()?;
            Some(Color::HexRGBA(r, g, b, a))
        }
        _ => None,
    }
}

#[test]
fn test_color() {
    let color = Color::HexRGB(255, 255, 255);
    assert_eq!(color.to_string(), "#ffffff".to_string());
    let color = Color::HSL(100.0, 50.0, 50.0);
    assert_eq!(color.to_string(), "hsl(100, 50%, 50%)".to_string());
}

#[test]
fn test_color_convert() {
    let color = Color::HSL(60.0, 0.0, 100.0);
    assert_eq!(color.to_rgb(), Color::HexRGB(255, 255, 255));
    let color = Color::HSL(0.0, 100.0, 50.0);
    assert_eq!(color.to_rgb(), Color::HexRGB(255, 0, 0));
}
