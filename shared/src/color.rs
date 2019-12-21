use std::fmt;

// TODO other color variants.
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::HexRGB(r, g, b) => write!(f, "#{:x}{:x}{:x}", r, g, b),
            Color::HexRGBA(r, g, b, a) => write!(f, "#{:x}{:x}{:x}{:x}", r, g, b, a),
            Color::HSL(h, s, l) => write!(f, "hsl({}, {}%, {}%)", h, s, l),
            Color::HSLA(h, s, l, a) => write!(f, "hsla({}, {}%, {}%, {})", h, s, l, a),
        }
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
}