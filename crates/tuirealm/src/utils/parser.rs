//! ## Parser
//!
//! This module exposes parsing utilities

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{Email, PhoneNumber};
use crate::tui::style::Color;

use regex::Regex;
use std::str::FromStr;

lazy_static! {
    /**
     * Regex matches:
     * - group 1: Red
     * - group 2: Green
     * - group 3: Blue
     */
    static ref COLOR_HEX_REGEX: Regex = Regex::new(r"#(:?[0-9a-fA-F]{2})(:?[0-9a-fA-F]{2})(:?[0-9a-fA-F]{2})").unwrap();
    /**
     * Regex matches:
     * - group 2: Red
     * - group 4: Green
     * - group 6: blue
     */
    static ref COLOR_RGB_REGEX: Regex = Regex::new(r"^(rgb)?\(?([01]?\d\d?|2[0-4]\d|25[0-5])(\W+)([01]?\d\d?|2[0-4]\d|25[0-5])\W+(([01]?\d\d?|2[0-4]\d|25[0-5])\)?)").unwrap();

    /**
     * Regex matches:
     * - group 1: name
     * - group 2: mail agent
     */
    static ref EMAIL_REGEX: Regex = Regex::new(r"^(:?[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+)@(:?[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$)").unwrap();

    /**
     * Regex matches:
     * - group 2|3: prefix
     * - group 4: number
     */
    static ref PHONE_NUMBER_REGEX: Regex = Regex::new(r"^([+]{1}(:?[0-9]{1,4})|[0]{2}(:?[0-9]{1,4}))?(:?[-\s\./0-9]*$)").unwrap();
}

/// ### parse_email
///
/// If provided string is a valid email address, returns the name and the mail agent
///
/// ## example
///
/// ```rust
/// use tuirealm::utils::parser::*;
/// use tuirealm::utils::Email;
/// assert_eq!(parse_email("christian.visintin@gmail.com").unwrap(), Email::new("christian.visintin", "gmail.com"));
/// ```
pub fn parse_email(s: &str) -> Option<Email> {
    match EMAIL_REGEX.captures(s) {
        None => None,
        Some(groups) => {
            let name = groups.get(1).map(|x| x.as_str())?;
            let agent = groups.get(2).map(|x| x.as_str())?;
            Some(Email::new(name, agent))
        }
    }
}

/// ### parse_phone_number
///
/// If provided string is a valid phone number address, returns the prefix (if any) and the number
///
/// ## example
///
/// ```rust
/// use tuirealm::utils::parser::*;
/// use tuirealm::utils::PhoneNumber;
/// assert_eq!(parse_phone_number("+39 345 777 6117").unwrap(), PhoneNumber::new(Some("39"), "3457776117"));
/// assert_eq!(parse_phone_number("0039 345 777 6117").unwrap(), PhoneNumber::new(Some("39"), "3457776117"));
/// ```
pub fn parse_phone_number(s: &str) -> Option<PhoneNumber> {
    match PHONE_NUMBER_REGEX.captures(s) {
        None => None,
        Some(groups) => {
            let prefix = match groups.get(2) {
                Some(p) => Some(p.as_str()),
                None => groups.get(3).map(|p| p.as_str()),
            };
            let number = groups.get(4).map(|x| x.as_str())?;
            Some(PhoneNumber::new(prefix, number))
        }
    }
}

/// ### parse_color
///
/// Parse color from string into a `Color` enum.
///
/// Color may be in different format:
///
/// 1. color name:
///     - Black,
///     - Blue,
///     - Cyan,
///     - DarkGray,
///     - Gray,
///     - Green,
///     - LightBlue,
///     - LightCyan,
///     - LightGreen,
///     - LightMagenta,
///     - LightRed,
///     - LightYellow,
///     - Magenta,
///     - Red,
///     - Reset,
///     - White,
///     - Yellow,
/// 2. Hex format:
///     - #f0ab05
///     - #AA33BC
/// 3. Rgb format:
///     - rgb(255, 64, 32)
///     - rgb(255,64,32)
///     - 255, 64, 32
pub fn parse_color(color: &str) -> Option<Color> {
    match color.to_lowercase().as_str() {
        // -- lib colors
        "black" => Some(Color::Black),
        "blue" => Some(Color::Blue),
        "cyan" => Some(Color::Cyan),
        "darkgray" | "darkgrey" => Some(Color::DarkGray),
        "default" => Some(Color::Reset),
        "gray" => Some(Color::Gray),
        "green" => Some(Color::Green),
        "lightblue" => Some(Color::LightBlue),
        "lightcyan" => Some(Color::LightCyan),
        "lightgreen" => Some(Color::LightGreen),
        "lightmagenta" => Some(Color::LightMagenta),
        "lightred" => Some(Color::LightRed),
        "lightyellow" => Some(Color::LightYellow),
        "magenta" => Some(Color::Magenta),
        "red" => Some(Color::Red),
        "white" => Some(Color::White),
        "yellow" => Some(Color::Yellow),
        // -- css colors
        "aliceblue" => Some(Color::Rgb(240, 248, 255)),
        "antiquewhite" => Some(Color::Rgb(250, 235, 215)),
        "aqua" => Some(Color::Rgb(0, 255, 255)),
        "aquamarine" => Some(Color::Rgb(127, 255, 212)),
        "azure" => Some(Color::Rgb(240, 255, 255)),
        "beige" => Some(Color::Rgb(245, 245, 220)),
        "bisque" => Some(Color::Rgb(255, 228, 196)),
        "blanchedalmond" => Some(Color::Rgb(255, 235, 205)),
        "blueviolet" => Some(Color::Rgb(138, 43, 226)),
        "brown" => Some(Color::Rgb(165, 42, 42)),
        "burlywood" => Some(Color::Rgb(222, 184, 135)),
        "cadetblue" => Some(Color::Rgb(95, 158, 160)),
        "chartreuse" => Some(Color::Rgb(127, 255, 0)),
        "chocolate" => Some(Color::Rgb(210, 105, 30)),
        "coral" => Some(Color::Rgb(255, 127, 80)),
        "cornflowerblue" => Some(Color::Rgb(100, 149, 237)),
        "cornsilk" => Some(Color::Rgb(255, 248, 220)),
        "crimson" => Some(Color::Rgb(220, 20, 60)),
        "darkblue" => Some(Color::Rgb(0, 0, 139)),
        "darkcyan" => Some(Color::Rgb(0, 139, 139)),
        "darkgoldenrod" => Some(Color::Rgb(184, 134, 11)),
        "darkgreen" => Some(Color::Rgb(0, 100, 0)),
        "darkkhaki" => Some(Color::Rgb(189, 183, 107)),
        "darkmagenta" => Some(Color::Rgb(139, 0, 139)),
        "darkolivegreen" => Some(Color::Rgb(85, 107, 47)),
        "darkorange" => Some(Color::Rgb(255, 140, 0)),
        "darkorchid" => Some(Color::Rgb(153, 50, 204)),
        "darkred" => Some(Color::Rgb(139, 0, 0)),
        "darksalmon" => Some(Color::Rgb(233, 150, 122)),
        "darkseagreen" => Some(Color::Rgb(143, 188, 143)),
        "darkslateblue" => Some(Color::Rgb(72, 61, 139)),
        "darkslategray" | "darkslategrey" => Some(Color::Rgb(47, 79, 79)),
        "darkturquoise" => Some(Color::Rgb(0, 206, 209)),
        "darkviolet" => Some(Color::Rgb(148, 0, 211)),
        "deeppink" => Some(Color::Rgb(255, 20, 147)),
        "deepskyblue" => Some(Color::Rgb(0, 191, 255)),
        "dimgray" | "dimgrey" => Some(Color::Rgb(105, 105, 105)),
        "dodgerblue" => Some(Color::Rgb(30, 144, 255)),
        "firebrick" => Some(Color::Rgb(178, 34, 34)),
        "floralwhite" => Some(Color::Rgb(255, 250, 240)),
        "forestgreen" => Some(Color::Rgb(34, 139, 34)),
        "fuchsia" => Some(Color::Rgb(255, 0, 255)),
        "gainsboro" => Some(Color::Rgb(220, 220, 220)),
        "ghostwhite" => Some(Color::Rgb(248, 248, 255)),
        "gold" => Some(Color::Rgb(255, 215, 0)),
        "goldenrod" => Some(Color::Rgb(218, 165, 32)),
        "greenyellow" => Some(Color::Rgb(173, 255, 47)),
        "grey" => Some(Color::Rgb(128, 128, 128)),
        "honeydew" => Some(Color::Rgb(240, 255, 240)),
        "hotpink" => Some(Color::Rgb(255, 105, 180)),
        "indianred" => Some(Color::Rgb(205, 92, 92)),
        "indigo" => Some(Color::Rgb(75, 0, 130)),
        "ivory" => Some(Color::Rgb(255, 255, 240)),
        "khaki" => Some(Color::Rgb(240, 230, 140)),
        "lavender" => Some(Color::Rgb(230, 230, 250)),
        "lavenderblush" => Some(Color::Rgb(255, 240, 245)),
        "lawngreen" => Some(Color::Rgb(124, 252, 0)),
        "lemonchiffon" => Some(Color::Rgb(255, 250, 205)),
        "lightcoral" => Some(Color::Rgb(240, 128, 128)),
        "lightgoldenrodyellow" => Some(Color::Rgb(250, 250, 210)),
        "lightgray" | "lightgrey" => Some(Color::Rgb(211, 211, 211)),
        "lightpink" => Some(Color::Rgb(255, 182, 193)),
        "lightsalmon" => Some(Color::Rgb(255, 160, 122)),
        "lightseagreen" => Some(Color::Rgb(32, 178, 170)),
        "lightskyblue" => Some(Color::Rgb(135, 206, 250)),
        "lightslategray" | "lightslategrey" => Some(Color::Rgb(119, 136, 153)),
        "lightsteelblue" => Some(Color::Rgb(176, 196, 222)),
        "lime" => Some(Color::Rgb(0, 255, 0)),
        "limegreen" => Some(Color::Rgb(50, 205, 50)),
        "linen" => Some(Color::Rgb(250, 240, 230)),
        "maroon" => Some(Color::Rgb(128, 0, 0)),
        "mediumaquamarine" => Some(Color::Rgb(102, 205, 170)),
        "mediumblue" => Some(Color::Rgb(0, 0, 205)),
        "mediumorchid" => Some(Color::Rgb(186, 85, 211)),
        "mediumpurple" => Some(Color::Rgb(147, 112, 219)),
        "mediumseagreen" => Some(Color::Rgb(60, 179, 113)),
        "mediumslateblue" => Some(Color::Rgb(123, 104, 238)),
        "mediumspringgreen" => Some(Color::Rgb(0, 250, 154)),
        "mediumturquoise" => Some(Color::Rgb(72, 209, 204)),
        "mediumvioletred" => Some(Color::Rgb(199, 21, 133)),
        "midnightblue" => Some(Color::Rgb(25, 25, 112)),
        "mintcream" => Some(Color::Rgb(245, 255, 250)),
        "mistyrose" => Some(Color::Rgb(255, 228, 225)),
        "moccasin" => Some(Color::Rgb(255, 228, 181)),
        "navajowhite" => Some(Color::Rgb(255, 222, 173)),
        "navy" => Some(Color::Rgb(0, 0, 128)),
        "oldlace" => Some(Color::Rgb(253, 245, 230)),
        "olive" => Some(Color::Rgb(128, 128, 0)),
        "olivedrab" => Some(Color::Rgb(107, 142, 35)),
        "orange" => Some(Color::Rgb(255, 165, 0)),
        "orangered" => Some(Color::Rgb(255, 69, 0)),
        "orchid" => Some(Color::Rgb(218, 112, 214)),
        "palegoldenrod" => Some(Color::Rgb(238, 232, 170)),
        "palegreen" => Some(Color::Rgb(152, 251, 152)),
        "paleturquoise" => Some(Color::Rgb(175, 238, 238)),
        "palevioletred" => Some(Color::Rgb(219, 112, 147)),
        "papayawhip" => Some(Color::Rgb(255, 239, 213)),
        "peachpuff" => Some(Color::Rgb(255, 218, 185)),
        "peru" => Some(Color::Rgb(205, 133, 63)),
        "pink" => Some(Color::Rgb(255, 192, 203)),
        "plum" => Some(Color::Rgb(221, 160, 221)),
        "powderblue" => Some(Color::Rgb(176, 224, 230)),
        "purple" => Some(Color::Rgb(128, 0, 128)),
        "rebeccapurple" => Some(Color::Rgb(102, 51, 153)),
        "rosybrown" => Some(Color::Rgb(188, 143, 143)),
        "royalblue" => Some(Color::Rgb(65, 105, 225)),
        "saddlebrown" => Some(Color::Rgb(139, 69, 19)),
        "salmon" => Some(Color::Rgb(250, 128, 114)),
        "sandybrown" => Some(Color::Rgb(244, 164, 96)),
        "seagreen" => Some(Color::Rgb(46, 139, 87)),
        "seashell" => Some(Color::Rgb(255, 245, 238)),
        "sienna" => Some(Color::Rgb(160, 82, 45)),
        "silver" => Some(Color::Rgb(192, 192, 192)),
        "skyblue" => Some(Color::Rgb(135, 206, 235)),
        "slateblue" => Some(Color::Rgb(106, 90, 205)),
        "slategray" | "slategrey" => Some(Color::Rgb(112, 128, 144)),
        "snow" => Some(Color::Rgb(255, 250, 250)),
        "springgreen" => Some(Color::Rgb(0, 255, 127)),
        "steelblue" => Some(Color::Rgb(70, 130, 180)),
        "tan" => Some(Color::Rgb(210, 180, 140)),
        "teal" => Some(Color::Rgb(0, 128, 128)),
        "thistle" => Some(Color::Rgb(216, 191, 216)),
        "tomato" => Some(Color::Rgb(255, 99, 71)),
        "turquoise" => Some(Color::Rgb(64, 224, 208)),
        "violet" => Some(Color::Rgb(238, 130, 238)),
        "wheat" => Some(Color::Rgb(245, 222, 179)),
        "whitesmoke" => Some(Color::Rgb(245, 245, 245)),
        "yellowgreen" => Some(Color::Rgb(154, 205, 50)),
        // -- hex and rgb
        other => {
            // Try as hex
            if let Some(color) = parse_hex_color(other) {
                Some(color)
            } else {
                parse_rgb_color(other)
            }
        }
    }
}

/// ### parse_hex_color
///
/// Try to parse a color in hex format, such as:
///
/// - "#f0ab05"
/// - "#AA33BC"
fn parse_hex_color(color: &str) -> Option<Color> {
    COLOR_HEX_REGEX.captures(color).map(|groups| {
        Color::Rgb(
            u8::from_str_radix(groups.get(1).unwrap().as_str(), 16)
                .ok()
                .unwrap(),
            u8::from_str_radix(groups.get(2).unwrap().as_str(), 16)
                .ok()
                .unwrap(),
            u8::from_str_radix(groups.get(3).unwrap().as_str(), 16)
                .ok()
                .unwrap(),
        )
    })
}

/// ### parse_rgb_color
///
/// Try to parse a color in rgb format, such as:
///
/// - "rgb(255, 64, 32)"
/// - "rgb(255,64,32)"
/// - "255, 64, 32"
fn parse_rgb_color(color: &str) -> Option<Color> {
    COLOR_RGB_REGEX.captures(color).map(|groups| {
        Color::Rgb(
            u8::from_str(groups.get(2).unwrap().as_str()).ok().unwrap(),
            u8::from_str(groups.get(4).unwrap().as_str()).ok().unwrap(),
            u8::from_str(groups.get(6).unwrap().as_str()).ok().unwrap(),
        )
    })
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn utils_parse_email() {
        assert_eq!(
            parse_email("christian.visintin@gmail.com").unwrap(),
            Email::new("christian.visintin", "gmail.com")
        );
        assert!(parse_email("pippo.pluto").is_none());
    }

    #[test]
    fn utils_parse_phone() {
        assert_eq!(
            parse_phone_number("+39 345 777 6117").unwrap(),
            PhoneNumber::new(Some("39"), "3457776117")
        );
        assert_eq!(
            parse_phone_number("0039 345 777 6117").unwrap(),
            PhoneNumber::new(Some("39"), "3457776117")
        );
        assert_eq!(
            parse_phone_number("345 777 6117").unwrap(),
            PhoneNumber::new(None, "3457776117")
        );
        assert!(parse_phone_number("nyc-safe").is_none());
    }

    #[test]
    fn utils_parse_color() {
        assert_eq!(parse_color("Black").unwrap(), Color::Black);
        assert_eq!(parse_color("BLUE").unwrap(), Color::Blue);
        assert_eq!(parse_color("Cyan").unwrap(), Color::Cyan);
        assert_eq!(parse_color("DarkGray").unwrap(), Color::DarkGray);
        assert_eq!(parse_color("Gray").unwrap(), Color::Gray);
        assert_eq!(parse_color("Green").unwrap(), Color::Green);
        assert_eq!(parse_color("LightBlue").unwrap(), Color::LightBlue);
        assert_eq!(parse_color("LightCyan").unwrap(), Color::LightCyan);
        assert_eq!(parse_color("LightGreen").unwrap(), Color::LightGreen);
        assert_eq!(parse_color("LightMagenta").unwrap(), Color::LightMagenta);
        assert_eq!(parse_color("LightRed").unwrap(), Color::LightRed);
        assert_eq!(parse_color("LightYellow").unwrap(), Color::LightYellow);
        assert_eq!(parse_color("Magenta").unwrap(), Color::Magenta);
        assert_eq!(parse_color("Red").unwrap(), Color::Red);
        assert_eq!(parse_color("Default").unwrap(), Color::Reset);
        assert_eq!(parse_color("White").unwrap(), Color::White);
        assert_eq!(parse_color("Yellow").unwrap(), Color::Yellow);
        assert_eq!(parse_color("#f0f0f0").unwrap(), Color::Rgb(240, 240, 240));
        // -- css colors
        assert_eq!(parse_color("aliceblue"), Some(Color::Rgb(240, 248, 255)));
        assert_eq!(parse_color("antiquewhite"), Some(Color::Rgb(250, 235, 215)));
        assert_eq!(parse_color("aqua"), Some(Color::Rgb(0, 255, 255)));
        assert_eq!(parse_color("aquamarine"), Some(Color::Rgb(127, 255, 212)));
        assert_eq!(parse_color("azure"), Some(Color::Rgb(240, 255, 255)));
        assert_eq!(parse_color("beige"), Some(Color::Rgb(245, 245, 220)));
        assert_eq!(parse_color("bisque"), Some(Color::Rgb(255, 228, 196)));
        assert_eq!(
            parse_color("blanchedalmond"),
            Some(Color::Rgb(255, 235, 205))
        );
        assert_eq!(parse_color("blueviolet"), Some(Color::Rgb(138, 43, 226)));
        assert_eq!(parse_color("brown"), Some(Color::Rgb(165, 42, 42)));
        assert_eq!(parse_color("burlywood"), Some(Color::Rgb(222, 184, 135)));
        assert_eq!(parse_color("cadetblue"), Some(Color::Rgb(95, 158, 160)));
        assert_eq!(parse_color("chartreuse"), Some(Color::Rgb(127, 255, 0)));
        assert_eq!(parse_color("chocolate"), Some(Color::Rgb(210, 105, 30)));
        assert_eq!(parse_color("coral"), Some(Color::Rgb(255, 127, 80)));
        assert_eq!(
            parse_color("cornflowerblue"),
            Some(Color::Rgb(100, 149, 237))
        );
        assert_eq!(parse_color("cornsilk"), Some(Color::Rgb(255, 248, 220)));
        assert_eq!(parse_color("crimson"), Some(Color::Rgb(220, 20, 60)));
        assert_eq!(parse_color("darkblue"), Some(Color::Rgb(0, 0, 139)));
        assert_eq!(parse_color("darkcyan"), Some(Color::Rgb(0, 139, 139)));
        assert_eq!(parse_color("darkgoldenrod"), Some(Color::Rgb(184, 134, 11)));
        assert_eq!(parse_color("darkgreen"), Some(Color::Rgb(0, 100, 0)));
        assert_eq!(parse_color("darkkhaki"), Some(Color::Rgb(189, 183, 107)));
        assert_eq!(parse_color("darkmagenta"), Some(Color::Rgb(139, 0, 139)));
        assert_eq!(parse_color("darkolivegreen"), Some(Color::Rgb(85, 107, 47)));
        assert_eq!(parse_color("darkorange"), Some(Color::Rgb(255, 140, 0)));
        assert_eq!(parse_color("darkorchid"), Some(Color::Rgb(153, 50, 204)));
        assert_eq!(parse_color("darkred"), Some(Color::Rgb(139, 0, 0)));
        assert_eq!(parse_color("darksalmon"), Some(Color::Rgb(233, 150, 122)));
        assert_eq!(parse_color("darkseagreen"), Some(Color::Rgb(143, 188, 143)));
        assert_eq!(parse_color("darkslateblue"), Some(Color::Rgb(72, 61, 139)));
        assert_eq!(parse_color("darkslategray"), Some(Color::Rgb(47, 79, 79)));
        assert_eq!(parse_color("darkslategrey"), Some(Color::Rgb(47, 79, 79)));
        assert_eq!(parse_color("darkturquoise"), Some(Color::Rgb(0, 206, 209)));
        assert_eq!(parse_color("darkviolet"), Some(Color::Rgb(148, 0, 211)));
        assert_eq!(parse_color("deeppink"), Some(Color::Rgb(255, 20, 147)));
        assert_eq!(parse_color("deepskyblue"), Some(Color::Rgb(0, 191, 255)));
        assert_eq!(parse_color("dimgray"), Some(Color::Rgb(105, 105, 105)));
        assert_eq!(parse_color("dimgrey"), Some(Color::Rgb(105, 105, 105)));
        assert_eq!(parse_color("dodgerblue"), Some(Color::Rgb(30, 144, 255)));
        assert_eq!(parse_color("firebrick"), Some(Color::Rgb(178, 34, 34)));
        assert_eq!(parse_color("floralwhite"), Some(Color::Rgb(255, 250, 240)));
        assert_eq!(parse_color("forestgreen"), Some(Color::Rgb(34, 139, 34)));
        assert_eq!(parse_color("fuchsia"), Some(Color::Rgb(255, 0, 255)));
        assert_eq!(parse_color("gainsboro"), Some(Color::Rgb(220, 220, 220)));
        assert_eq!(parse_color("ghostwhite"), Some(Color::Rgb(248, 248, 255)));
        assert_eq!(parse_color("gold"), Some(Color::Rgb(255, 215, 0)));
        assert_eq!(parse_color("goldenrod"), Some(Color::Rgb(218, 165, 32)));
        assert_eq!(parse_color("greenyellow"), Some(Color::Rgb(173, 255, 47)));
        assert_eq!(parse_color("honeydew"), Some(Color::Rgb(240, 255, 240)));
        assert_eq!(parse_color("hotpink"), Some(Color::Rgb(255, 105, 180)));
        assert_eq!(parse_color("indianred"), Some(Color::Rgb(205, 92, 92)));
        assert_eq!(parse_color("indigo"), Some(Color::Rgb(75, 0, 130)));
        assert_eq!(parse_color("ivory"), Some(Color::Rgb(255, 255, 240)));
        assert_eq!(parse_color("khaki"), Some(Color::Rgb(240, 230, 140)));
        assert_eq!(parse_color("lavender"), Some(Color::Rgb(230, 230, 250)));
        assert_eq!(
            parse_color("lavenderblush"),
            Some(Color::Rgb(255, 240, 245))
        );
        assert_eq!(parse_color("lawngreen"), Some(Color::Rgb(124, 252, 0)));
        assert_eq!(parse_color("lemonchiffon"), Some(Color::Rgb(255, 250, 205)));
        assert_eq!(parse_color("lightcoral"), Some(Color::Rgb(240, 128, 128)));
        assert_eq!(
            parse_color("lightgoldenrodyellow"),
            Some(Color::Rgb(250, 250, 210))
        );
        assert_eq!(parse_color("lightpink"), Some(Color::Rgb(255, 182, 193)));
        assert_eq!(parse_color("lightsalmon"), Some(Color::Rgb(255, 160, 122)));
        assert_eq!(parse_color("lightseagreen"), Some(Color::Rgb(32, 178, 170)));
        assert_eq!(parse_color("lightskyblue"), Some(Color::Rgb(135, 206, 250)));
        assert_eq!(
            parse_color("lightslategray"),
            Some(Color::Rgb(119, 136, 153))
        );
        assert_eq!(
            parse_color("lightslategrey"),
            Some(Color::Rgb(119, 136, 153))
        );
        assert_eq!(
            parse_color("lightsteelblue"),
            Some(Color::Rgb(176, 196, 222))
        );
        assert_eq!(parse_color("lime"), Some(Color::Rgb(0, 255, 0)));
        assert_eq!(parse_color("limegreen"), Some(Color::Rgb(50, 205, 50)));
        assert_eq!(parse_color("linen"), Some(Color::Rgb(250, 240, 230)));
        assert_eq!(parse_color("maroon"), Some(Color::Rgb(128, 0, 0)));
        assert_eq!(
            parse_color("mediumaquamarine"),
            Some(Color::Rgb(102, 205, 170))
        );
        assert_eq!(parse_color("mediumblue"), Some(Color::Rgb(0, 0, 205)));
        assert_eq!(parse_color("mediumorchid"), Some(Color::Rgb(186, 85, 211)));
        assert_eq!(parse_color("mediumpurple"), Some(Color::Rgb(147, 112, 219)));
        assert_eq!(
            parse_color("mediumseagreen"),
            Some(Color::Rgb(60, 179, 113))
        );
        assert_eq!(
            parse_color("mediumslateblue"),
            Some(Color::Rgb(123, 104, 238))
        );
        assert_eq!(
            parse_color("mediumspringgreen"),
            Some(Color::Rgb(0, 250, 154))
        );
        assert_eq!(
            parse_color("mediumturquoise"),
            Some(Color::Rgb(72, 209, 204))
        );
        assert_eq!(
            parse_color("mediumvioletred"),
            Some(Color::Rgb(199, 21, 133))
        );
        assert_eq!(parse_color("midnightblue"), Some(Color::Rgb(25, 25, 112)));
        assert_eq!(parse_color("mintcream"), Some(Color::Rgb(245, 255, 250)));
        assert_eq!(parse_color("mistyrose"), Some(Color::Rgb(255, 228, 225)));
        assert_eq!(parse_color("moccasin"), Some(Color::Rgb(255, 228, 181)));
        assert_eq!(parse_color("navajowhite"), Some(Color::Rgb(255, 222, 173)));
        assert_eq!(parse_color("navy"), Some(Color::Rgb(0, 0, 128)));
        assert_eq!(parse_color("oldlace"), Some(Color::Rgb(253, 245, 230)));
        assert_eq!(parse_color("olive"), Some(Color::Rgb(128, 128, 0)));
        assert_eq!(parse_color("olivedrab"), Some(Color::Rgb(107, 142, 35)));
        assert_eq!(parse_color("orange"), Some(Color::Rgb(255, 165, 0)));
        assert_eq!(parse_color("orangered"), Some(Color::Rgb(255, 69, 0)));
        assert_eq!(parse_color("orchid"), Some(Color::Rgb(218, 112, 214)));
        assert_eq!(
            parse_color("palegoldenrod"),
            Some(Color::Rgb(238, 232, 170))
        );
        assert_eq!(parse_color("palegreen"), Some(Color::Rgb(152, 251, 152)));
        assert_eq!(
            parse_color("paleturquoise"),
            Some(Color::Rgb(175, 238, 238))
        );
        assert_eq!(
            parse_color("palevioletred"),
            Some(Color::Rgb(219, 112, 147))
        );
        assert_eq!(parse_color("papayawhip"), Some(Color::Rgb(255, 239, 213)));
        assert_eq!(parse_color("peachpuff"), Some(Color::Rgb(255, 218, 185)));
        assert_eq!(parse_color("peru"), Some(Color::Rgb(205, 133, 63)));
        assert_eq!(parse_color("pink"), Some(Color::Rgb(255, 192, 203)));
        assert_eq!(parse_color("plum"), Some(Color::Rgb(221, 160, 221)));
        assert_eq!(parse_color("powderblue"), Some(Color::Rgb(176, 224, 230)));
        assert_eq!(parse_color("purple"), Some(Color::Rgb(128, 0, 128)));
        assert_eq!(parse_color("rebeccapurple"), Some(Color::Rgb(102, 51, 153)));
        assert_eq!(parse_color("rosybrown"), Some(Color::Rgb(188, 143, 143)));
        assert_eq!(parse_color("royalblue"), Some(Color::Rgb(65, 105, 225)));
        assert_eq!(parse_color("saddlebrown"), Some(Color::Rgb(139, 69, 19)));
        assert_eq!(parse_color("salmon"), Some(Color::Rgb(250, 128, 114)));
        assert_eq!(parse_color("sandybrown"), Some(Color::Rgb(244, 164, 96)));
        assert_eq!(parse_color("seagreen"), Some(Color::Rgb(46, 139, 87)));
        assert_eq!(parse_color("seashell"), Some(Color::Rgb(255, 245, 238)));
        assert_eq!(parse_color("sienna"), Some(Color::Rgb(160, 82, 45)));
        assert_eq!(parse_color("silver"), Some(Color::Rgb(192, 192, 192)));
        assert_eq!(parse_color("skyblue"), Some(Color::Rgb(135, 206, 235)));
        assert_eq!(parse_color("slateblue"), Some(Color::Rgb(106, 90, 205)));
        assert_eq!(parse_color("slategray"), Some(Color::Rgb(112, 128, 144)));
        assert_eq!(parse_color("slategrey"), Some(Color::Rgb(112, 128, 144)));
        assert_eq!(parse_color("snow"), Some(Color::Rgb(255, 250, 250)));
        assert_eq!(parse_color("springgreen"), Some(Color::Rgb(0, 255, 127)));
        assert_eq!(parse_color("steelblue"), Some(Color::Rgb(70, 130, 180)));
        assert_eq!(parse_color("tan"), Some(Color::Rgb(210, 180, 140)));
        assert_eq!(parse_color("teal"), Some(Color::Rgb(0, 128, 128)));
        assert_eq!(parse_color("thistle"), Some(Color::Rgb(216, 191, 216)));
        assert_eq!(parse_color("tomato"), Some(Color::Rgb(255, 99, 71)));
        assert_eq!(parse_color("turquoise"), Some(Color::Rgb(64, 224, 208)));
        assert_eq!(parse_color("violet"), Some(Color::Rgb(238, 130, 238)));
        assert_eq!(parse_color("wheat"), Some(Color::Rgb(245, 222, 179)));
        assert_eq!(parse_color("whitesmoke"), Some(Color::Rgb(245, 245, 245)));
        assert_eq!(parse_color("yellowgreen"), Some(Color::Rgb(154, 205, 50)));
        // -- hex and rgb
        assert_eq!(
            parse_color("rgb(255, 64, 32)").unwrap(),
            Color::Rgb(255, 64, 32)
        );
        assert!(parse_color("redd").is_none());
    }
}
