//! Utilities for working with colors and color palettes.

use num_traits::{Bounded, FromPrimitive, NumCast};
use std::fmt;

/// A construct that can be treated as encoding a solid color.
pub trait SolidColor<T: NumCast + Bounded>: Color<T> + Into<Rgb<T>> + Into<Hsl<T>> {
    /// Converts the color to RGB.
    fn to_rgb(self) -> Rgb<T> {
        self.into()
    }
    /// Converts the color to HSL.
    fn to_hsl(self) -> Hsl<T> {
        self.into()
    }
    /// Converts the color to RGBA, adding a full alpha value.
    fn to_rgba(self) -> Rgba<T> {
        let rgb: Rgb<T> = self.into();
        Rgba::<T>(rgb.0, rgb.1, rgb.2, T::max_value())
    }
    /// Converts the color to HSLA, adding a full alpha value.
    fn to_hsla(self) -> Hsla<T> {
        let hsl: Hsl<T> = self.into();
        Hsla::<T>(hsl.0, hsl.1, hsl.2, T::max_value())
    }
}

/// A construct that can be treated as encoding a general color.
///
/// By nature, this requires the construct to encode an alpha value.
pub trait Color<T: NumCast + Bounded>: Into<Rgba<T>> + Into<Hsla<T>> {
    /// Converts the color to RGBA.
    fn to_rgba(self) -> Rgba<T> {
        self.into()
    }
    /// Converts the color to HSLA.
    fn to_hsla(self) -> Hsla<T> {
        self.into()
    }
}

#[inline(always)]
fn _max<T: PartialOrd>(l: T, r: T) -> T {
    if r > l {
        r
    } else {
        l
    }
}

#[inline(always)]
fn _min<T: PartialOrd>(l: T, r: T) -> T {
    if r < l {
        r
    } else {
        l
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Rgb<T>> for Hsl<T> {
    fn from(other: Rgb<T>) -> Self {
        let triple = (
            other.0.to_f64().unwrap_or_default(),
            other.1.to_f64().unwrap_or_default(),
            other.2.to_f64().unwrap_or_default(),
        );
        let max = T::max_value().to_f64().unwrap_or(1.0);
        let (r, g, b) = (triple.0 / max, triple.1 / max, triple.2 / max);
        let max = _max(_max(r, g), b);
        let min = _min(_min(r, g), b);
        let delta = max - min;
        let l = (max + min) / 2.0;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        let h: f64 = if delta == 0.0 {
            0.0
        } else if max == g {
            (((b - r) / delta) + 2.0) / 6.0
        } else if max == b {
            (((r - g) / delta) + 4.0) / 6.0
        } else if max == r {
            (((g - b) / delta) % 6.0) / 6.0
        } else {
            unreachable!()
        };
        Hsl::<T>(
            T::from_f64(T::max_value().to_f64().unwrap_or(1.0) * h).unwrap_or_default(),
            T::from_f64(T::max_value().to_f64().unwrap_or(1.0) * s).unwrap_or_default(),
            T::from_f64(T::max_value().to_f64().unwrap_or(1.0) * l).unwrap_or_default(),
        )
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Hsl<T>> for Rgb<T> {
    fn from(other: Hsl<T>) -> Self {
        let triple = (
            other.0.to_f64().unwrap_or_default(),
            other.1.to_f64().unwrap_or_default(),
            other.2.to_f64().unwrap_or_default(),
        );
        let max = T::max_value().to_f64().unwrap_or(1.0);
        let (h, s, l) = (triple.0 / max, triple.1 / max, triple.2 / max);
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - (((h * 6.0) % 2.0) - 1.0).abs());
        let m = l - c / 2.0;
        let f_h = h * 6.0;
        let (r, g, b) = if f_h >= 1.0 && f_h < 2.0 {
            (x, c, 0.0)
        } else if f_h >= 2.0 && f_h < 3.0 {
            (0.0, c, x)
        } else if f_h >= 3.0 && f_h < 4.0 {
            (0.0, x, c)
        } else if f_h >= 4.0 && f_h < 5.0 {
            (x, 0.0, c)
        } else if f_h >= 5.0 && f_h < 6.0 {
            (c, 0.0, x)
        } else {
            (c, x, 0.0)
        };
        let (r, g, b) = (r + m, g + m, b + m);
        Rgb::<T>(
            T::from_f64((T::max_value().to_f64().unwrap_or(1.0) * r).ceil()).unwrap_or_default(),
            T::from_f64((T::max_value().to_f64().unwrap_or(1.0) * g).ceil()).unwrap_or_default(),
            T::from_f64((T::max_value().to_f64().unwrap_or(1.0) * b).ceil()).unwrap_or_default(),
        )
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Hsla<T>> for Rgba<T> {
    fn from(other: Hsla<T>) -> Self {
        let hsl = Hsl::<T>(other.0, other.1, other.2);
        let rgb: Rgb<T> = hsl.into();
        Rgba::<T>(rgb.0, rgb.1, rgb.2, other.3)
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Rgba<T>> for Hsla<T> {
    fn from(other: Rgba<T>) -> Self {
        let rgb = Rgb::<T>(other.0, other.1, other.2);
        let hsl: Hsl<T> = rgb.into();
        Hsla::<T>(hsl.0, hsl.1, hsl.2, other.3)
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Rgb<T>> for Rgba<T> {
    fn from(other: Rgb<T>) -> Self {
        Rgba::<T>(other.0, other.1, other.2, T::max_value())
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Hsl<T>> for Hsla<T> {
    fn from(other: Hsl<T>) -> Self {
        Hsla::<T>(other.0, other.1, other.2, T::max_value())
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Rgb<T>> for Hsla<T> {
    fn from(other: Rgb<T>) -> Self {
        let hsl: Hsl<T> = other.into();
        Hsla::<T>(hsl.0, hsl.1, hsl.2, T::max_value())
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> From<Hsl<T>> for Rgba<T> {
    fn from(other: Hsl<T>) -> Self {
        let rgb: Rgb<T> = other.into();
        Rgba::<T>(rgb.0, rgb.1, rgb.2, T::max_value())
    }
}

/// Fetch a solid color by name.
pub trait Name: Sized {
    /// Returns the color associated with the name, if it exists.
    fn with_name(name: &str) -> Option<Self>;
}

impl<T: NumCast + Bounded + FromPrimitive + Default> Name for Rgb<T> {
    fn with_name(name: &str) -> Option<Self> {
        let components: Option<(u8, u8, u8)> = match name.to_lowercase().as_ref() {
            "black" => Some((0x00, 0x00, 0x00)),
            "silver" => Some((0xc0, 0xc0, 0xc0)),
            "gray" => Some((0x80, 0x80, 0x80)),
            "white" => Some((0xff, 0xff, 0xff)),
            "maroon" => Some((0x80, 0x00, 0x00)),
            "red" => Some((0xff, 0x00, 0x00)),
            "purple" => Some((0x80, 0x00, 0x80)),
            "fuchsia" => Some((0xff, 0x00, 0xff)),
            "green" => Some((0x00, 0x80, 0x00)),
            "lime" => Some((0x00, 0xff, 0x00)),
            "olive" => Some((0x80, 0x80, 0x00)),
            "yellow" => Some((0xff, 0xff, 0x00)),
            "navy" => Some((0x00, 0x00, 0x80)),
            "blue" => Some((0x00, 0x00, 0xff)),
            "teal" => Some((0x00, 0x80, 0x80)),
            "aqua" => Some((0x00, 0xff, 0xff)),
            "orange" => Some((0xff, 0xa5, 0x00)),
            "aliceblue" => Some((0xf0, 0xf8, 0xff)),
            "antiquewhite" => Some((0xfa, 0xeb, 0xd7)),
            "aquamarine" => Some((0x7f, 0xff, 0xd4)),
            "azure" => Some((0xf0, 0xff, 0xff)),
            "beige" => Some((0xf5, 0xf5, 0xdc)),
            "bisque" => Some((0xff, 0xe4, 0xc4)),
            "blanchedalmond" => Some((0xff, 0xeb, 0xcd)),
            "blueviolet" => Some((0x8a, 0x2b, 0xe2)),
            "brown" => Some((0xa5, 0x2a, 0x2a)),
            "burlywood" => Some((0xde, 0xb8, 0x87)),
            "cadetblue" => Some((0x5f, 0x9e, 0xa0)),
            "chartreuse" => Some((0x7f, 0xff, 0x00)),
            "chocolate" => Some((0xd2, 0x69, 0x1e)),
            "coral" => Some((0xff, 0x7f, 0x50)),
            "cornflowerblue" => Some((0x64, 0x95, 0xed)),
            "cornsilk" => Some((0xff, 0xf8, 0xdc)),
            "crimson" => Some((0xdc, 0x14, 0x3c)),
            "cyan" => Some((0x00, 0xff, 0xff)),
            "darkblue" => Some((0x00, 0x00, 0x8b)),
            "darkcyan" => Some((0x00, 0x8b, 0x8b)),
            "darkgoldenrod" => Some((0xb8, 0x86, 0x0b)),
            "darkgray" => Some((0xa9, 0xa9, 0xa9)),
            "darkgreen" => Some((0x00, 0x64, 0x00)),
            "darkgrey" => Some((0xa9, 0xa9, 0xa9)),
            "darkkhaki" => Some((0xbd, 0xb7, 0x6b)),
            "darkmagenta" => Some((0x8b, 0x00, 0x8b)),
            "darkolivegreen" => Some((0x55, 0x6b, 0x2f)),
            "darkorange" => Some((0xff, 0x8c, 0x00)),
            "darkorchid" => Some((0x99, 0x32, 0xcc)),
            "darkred" => Some((0x8b, 0x00, 0x00)),
            "darksalmon" => Some((0xe9, 0x96, 0x7a)),
            "darkseagreen" => Some((0x8f, 0xbc, 0x8f)),
            "darkslateblue" => Some((0x48, 0x3d, 0x8b)),
            "darkslategray" => Some((0x2f, 0x4f, 0x4f)),
            "darkslategrey" => Some((0x2f, 0x4f, 0x4f)),
            "darkturquoise" => Some((0x00, 0xce, 0xd1)),
            "darkviolet" => Some((0x94, 0x00, 0xd3)),
            "deeppink" => Some((0xff, 0x14, 0x93)),
            "deepskyblue" => Some((0x00, 0xbf, 0xff)),
            "dimgray" => Some((0x69, 0x69, 0x69)),
            "dimgrey" => Some((0x69, 0x69, 0x69)),
            "dodgerblue" => Some((0x1e, 0x90, 0xff)),
            "firebrick" => Some((0xb2, 0x22, 0x22)),
            "floralwhite" => Some((0xff, 0xfa, 0xf0)),
            "forestgreen" => Some((0x22, 0x8b, 0x22)),
            "gainsboro" => Some((0xdc, 0xdc, 0xdc)),
            "ghostwhite" => Some((0xf8, 0xf8, 0xff)),
            "gold" => Some((0xff, 0xd7, 0x00)),
            "goldenrod" => Some((0xda, 0xa5, 0x20)),
            "greenyellow" => Some((0xad, 0xff, 0x2f)),
            "grey" => Some((0x80, 0x80, 0x80)),
            "honeydew" => Some((0xf0, 0xff, 0xf0)),
            "hotpink" => Some((0xff, 0x69, 0xb4)),
            "indianred" => Some((0xcd, 0x5c, 0x5c)),
            "indigo" => Some((0x4b, 0x00, 0x82)),
            "ivory" => Some((0xff, 0xff, 0xf0)),
            "khaki" => Some((0xf0, 0xe6, 0x8c)),
            "lavender" => Some((0xe6, 0xe6, 0xfa)),
            "lavenderblush" => Some((0xff, 0xf0, 0xf5)),
            "lawngreen" => Some((0x7c, 0xfc, 0x00)),
            "lemonchiffon" => Some((0xff, 0xfa, 0xcd)),
            "lightblue" => Some((0xad, 0xd8, 0xe6)),
            "lightcoral" => Some((0xf0, 0x80, 0x80)),
            "lightcyan" => Some((0xe0, 0xff, 0xff)),
            "lightgoldenrodyellow" => Some((0xfa, 0xfa, 0xd2)),
            "lightgray" => Some((0xd3, 0xd3, 0xd3)),
            "lightgreen" => Some((0x90, 0xee, 0x90)),
            "lightgrey" => Some((0xd3, 0xd3, 0xd3)),
            "lightpink" => Some((0xff, 0xb6, 0xc1)),
            "lightsalmon" => Some((0xff, 0xa0, 0x7a)),
            "lightseagreen" => Some((0x20, 0xb2, 0xaa)),
            "lightskyblue" => Some((0x87, 0xce, 0xfa)),
            "lightslategray" => Some((0x77, 0x88, 0x99)),
            "lightslategrey" => Some((0x77, 0x88, 0x99)),
            "lightsteelblue" => Some((0xb0, 0xc4, 0xde)),
            "lightyellow" => Some((0xff, 0xff, 0xe0)),
            "limegreen" => Some((0x32, 0xcd, 0x32)),
            "linen" => Some((0xfa, 0xf0, 0xe6)),
            "magenta" => Some((0xff, 0x00, 0xff)),
            "mediumaquamarine" => Some((0x66, 0xcd, 0xaa)),
            "mediumblue" => Some((0x00, 0x00, 0xcd)),
            "mediumorchid" => Some((0xba, 0x55, 0xd3)),
            "mediumpurple" => Some((0x93, 0x70, 0xdb)),
            "mediumseagreen" => Some((0x3c, 0xb3, 0x71)),
            "mediumslateblue" => Some((0x7b, 0x68, 0xee)),
            "mediumspringgreen" => Some((0x00, 0xfa, 0x9a)),
            "mediumturquoise" => Some((0x48, 0xd1, 0xcc)),
            "mediumvioletred" => Some((0xc7, 0x15, 0x85)),
            "midnightblue" => Some((0x19, 0x19, 0x70)),
            "mintcream" => Some((0xf5, 0xff, 0xfa)),
            "mistyrose" => Some((0xff, 0xe4, 0xe1)),
            "moccasin" => Some((0xff, 0xe4, 0xb5)),
            "navajowhite" => Some((0xff, 0xde, 0xad)),
            "oldlace" => Some((0xfd, 0xf5, 0xe6)),
            "olivedrab" => Some((0x6b, 0x8e, 0x23)),
            "orangered" => Some((0xff, 0x45, 0x00)),
            "orchid" => Some((0xda, 0x70, 0xd6)),
            "palegoldenrod" => Some((0xee, 0xe8, 0xaa)),
            "palegreen" => Some((0x98, 0xfb, 0x98)),
            "paleturquoise" => Some((0xaf, 0xee, 0xee)),
            "palevioletred" => Some((0xdb, 0x70, 0x93)),
            "papayawhip" => Some((0xff, 0xef, 0xd5)),
            "peachpuff" => Some((0xff, 0xda, 0xb9)),
            "peru" => Some((0xcd, 0x85, 0x3f)),
            "pink" => Some((0xff, 0xc0, 0xcb)),
            "plum" => Some((0xdd, 0xa0, 0xdd)),
            "powderblue" => Some((0xb0, 0xe0, 0xe6)),
            "rosybrown" => Some((0xbc, 0x8f, 0x8f)),
            "royalblue" => Some((0x41, 0x69, 0xe1)),
            "saddlebrown" => Some((0x8b, 0x45, 0x13)),
            "salmon" => Some((0xfa, 0x80, 0x72)),
            "sandybrown" => Some((0xf4, 0xa4, 0x60)),
            "seagreen" => Some((0x2e, 0x8b, 0x57)),
            "seashell" => Some((0xff, 0xf5, 0xee)),
            "sienna" => Some((0xa0, 0x52, 0x2d)),
            "skyblue" => Some((0x87, 0xce, 0xeb)),
            "slateblue" => Some((0x6a, 0x5a, 0xcd)),
            "slategray" => Some((0x70, 0x80, 0x90)),
            "slategrey" => Some((0x70, 0x80, 0x90)),
            "snow" => Some((0xff, 0xfa, 0xfa)),
            "springgreen" => Some((0x00, 0xff, 0x7f)),
            "steelblue" => Some((0x46, 0x82, 0xb4)),
            "tan" => Some((0xd2, 0xb4, 0x8c)),
            "thistle" => Some((0xd8, 0xbf, 0xd8)),
            "tomato" => Some((0xff, 0x63, 0x47)),
            "turquoise" => Some((0x40, 0xe0, 0xd0)),
            "violet" => Some((0xee, 0x82, 0xee)),
            "wheat" => Some((0xf5, 0xde, 0xb3)),
            "whitesmoke" => Some((0xf5, 0xf5, 0xf5)),
            "yellowgreen" => Some((0x9a, 0xcd, 0x32)),
            "rebeccapurple" => Some((0x66, 0x33, 0x99)),
            _ => None,
        };
        components.map(|components| {
            let normalized = (
                components.0 as f64,
                components.1 as f64,
                components.2 as f64,
            );
            Rgb::<T>(
                T::from_f64(T::max_value().to_f64().unwrap_or(1.0) * normalized.0)
                    .unwrap_or_default(),
                T::from_f64(T::max_value().to_f64().unwrap_or(1.0) * normalized.1)
                    .unwrap_or_default(),
                T::from_f64(T::max_value().to_f64().unwrap_or(1.0) * normalized.2)
                    .unwrap_or_default(),
            )
        })
    }
}

impl<T: NumCast + Bounded + FromPrimitive + Default> Name for Hsl<T> {
    fn with_name(name: &str) -> Option<Self> {
        Rgb::<T>::with_name(name).map(|c| c.into())
    }
}

/// A color specified using a name.
#[derive(Clone, Debug)]
pub struct Named(String);

/// A color specified using red, green, and blue components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb<T>(T, T, T);

/// A color specified using red, green, blue, and alpha components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba<T>(T, T, T, T);

/// A color specified using hue, saturation, and lightness components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl<T>(T, T, T);

/// A color specified using hue, saturation, lightness, and alpha components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsla<T>(T, T, T, T);

impl<T: NumCast + Bounded + FromPrimitive + Default> SolidColor<T> for Rgb<T> {}
impl<T: NumCast + Bounded + FromPrimitive + Default> SolidColor<T> for Hsl<T> {}
impl<T: NumCast + Bounded + FromPrimitive + Default> Color<T> for Rgb<T> {}
impl<T: NumCast + Bounded + FromPrimitive + Default> Color<T> for Hsl<T> {}
impl<T: NumCast + Bounded + FromPrimitive + Default> Color<T> for Rgba<T> {}
impl<T: NumCast + Bounded + FromPrimitive + Default> Color<T> for Hsla<T> {}

// TODO(#21): Make at least fmt::Display CSS-compatible.
macro_rules! fmt {
    ($style:ident) => {
        impl<T: fmt::$style> fmt::$style for Rgb<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "rgb(")?;
                self.0.fmt(f)?;
                write!(f, ", ")?;
                self.1.fmt(f)?;
                write!(f, ", ")?;
                self.2.fmt(f)?;
                write!(f, ")")
            }
        }
        impl<T: fmt::$style> fmt::$style for Hsl<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "hsl(")?;
                self.0.fmt(f)?;
                write!(f, ", ")?;
                self.1.fmt(f)?;
                write!(f, ", ")?;
                self.2.fmt(f)?;
                write!(f, ")")
            }
        }
        impl<T: fmt::$style> fmt::$style for Rgba<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "rgba(")?;
                self.0.fmt(f)?;
                write!(f, ", ")?;
                self.1.fmt(f)?;
                write!(f, ", ")?;
                self.2.fmt(f)?;
                write!(f, ", ")?;
                self.3.fmt(f)?;
                write!(f, ")")
            }
        }
        impl<T: fmt::$style> fmt::$style for Hsla<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "hsla(")?;
                self.0.fmt(f)?;
                write!(f, ", ")?;
                self.1.fmt(f)?;
                write!(f, ", ")?;
                self.2.fmt(f)?;
                write!(f, ", ")?;
                self.3.fmt(f)?;
                write!(f, ")")
            }
        }
    };
}

fmt!(Binary);
fmt!(Display);
fmt!(LowerExp);
fmt!(LowerHex);
fmt!(Octal);
fmt!(UpperExp);
fmt!(UpperHex);

#[cfg(test)]
mod tests {
    use ui::color::*;
    #[test]
    fn test_rgb_to_hsl() {
        let rgb_colors = [
            (0, 0, 0),
            (255, 255, 255),
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (0, 255, 255),
            (255, 0, 255),
            (192, 192, 192),
            (128, 128, 128),
            (128, 0, 0),
            (128, 128, 0),
            (0, 128, 0),
            (128, 0, 128),
            (0, 128, 128),
            (0, 0, 128),
        ];
        let hsl_colors = [
            (0, 0, 0),
            (0, 0, 255),
            (0, 255, 127),
            (85, 255, 127),
            (170, 255, 127),
            (42, 255, 127),
            (127, 255, 127),
            (212, 255, 127),
            (0, 0, 192),
            (0, 0, 128),
            (0, 255, 64),
            (42, 255, 64),
            (85, 255, 64),
            (212, 255, 64),
            (127, 255, 64),
            (170, 255, 64),
        ];
        let pairs = rgb_colors.into_iter().zip(hsl_colors.into_iter()).map(
            |(rgb_color, hsl_color)| {
                let rgb = Rgb::<u8>(rgb_color.0, rgb_color.1, rgb_color.2);
                let hsl: Hsl<u8> = rgb.into();
                (hsl, Hsl::<u8>(hsl_color.0, hsl_color.1, hsl_color.2))
            },
        );
        for pair in pairs {
            assert_eq!(pair.0, pair.1);
        }
    }
    // TODO(#20): Re-enable when greater accuracy has been achieved.
    #[ignore]
    #[test]
    fn test_hsl_to_rgb() {
        let rgb_colors = [
            (0, 0, 0),
            (255, 255, 255),
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (0, 255, 255),
            (255, 0, 255),
            (192, 192, 192),
            (128, 128, 128),
            (128, 0, 0),
            (128, 128, 0),
            (0, 128, 0),
            (128, 0, 128),
            (0, 128, 128),
            (0, 0, 128),
        ];
        let hsl_colors = [
            (0, 0, 0),
            (0, 0, 255),
            (0, 255, 127),
            (85, 255, 127),
            (170, 255, 127),
            (42, 255, 127),
            (127, 255, 127),
            (212, 255, 127),
            (0, 0, 192),
            (0, 0, 128),
            (0, 255, 64),
            (42, 255, 64),
            (85, 255, 64),
            (212, 255, 64),
            (127, 255, 64),
            (170, 255, 64),
        ];
        let pairs = rgb_colors.into_iter().zip(hsl_colors.into_iter()).map(
            |(rgb_color, hsl_color)| {
                let hsl = Hsl::<u8>(hsl_color.0, hsl_color.1, hsl_color.2);
                let rgb: Rgb<u8> = hsl.into();
                (rgb, Rgb::<u8>(rgb_color.0, rgb_color.1, rgb_color.2))
            },
        );
        for pair in pairs {
            assert_eq!(pair.0, pair.1);
        }
    }
}
