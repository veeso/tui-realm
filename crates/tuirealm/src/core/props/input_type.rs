//! ## InputType
//!
//! This module exposes the input type

use crate::utils::parser::{parse_color, parse_email, parse_phone_number};

use std::fmt;
use std::str::FromStr;

/// Input type for text inputs
#[derive(Clone)]
pub enum InputType {
    /// Color input. Syntax can be css-color, `rgb(rrr, ggg, bbb)` or `#rrggbb`
    Color,
    /// Email input. Must contain a valid email address
    Email,
    /// Number. Can be signed and float
    Number,
    /// Text field; text will be shadowed by provided char
    Password(char),
    /// Telephone number. Syntax is (00|+) number
    Telephone,
    /// Plain text
    Text,
    /// Signed integer
    SignedInteger,
    /// Unsigned positive number
    UnsignedInteger,
    /// Custom field; displayed as plain text.
    /// You must provide the function to call on `validate` and the function to call on `char_valid`
    /// The `validate()` callback, is used to tell whether the entire input value is valid,
    /// while the `char_valid()` callback is used to tell whether the input char is allowed to be pushed to input value
    /// (e.g. an email address is `valid` if contains the name, the '@' and the domain;
    /// but `char_valid` allows characters, numbers, symbol and up to one '@')
    Custom(fn(&str) -> bool, fn(&str, char) -> bool),
    /// Custom validation password; text will be shadowed by provided char
    /// You must provide the function to call on `validate` and the function to call on `char_valid`.
    /// See `Custom` for callbacks meaning
    CustomPassword(char, fn(&str) -> bool, fn(&str, char) -> bool),
}

impl PartialEq for InputType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Color, Self::Color) => true,
            (Self::Email, Self::Email) => true,
            (Self::Number, Self::Number) => true,
            (Self::Password(ch), Self::Password(ch2)) => ch == ch2,
            (Self::Telephone, Self::Telephone) => true,
            (Self::Text, Self::Text) => true,
            (Self::SignedInteger, Self::SignedInteger) => true,
            (Self::UnsignedInteger, Self::UnsignedInteger) => true,
            (Self::Custom(..), Self::Custom(..)) => true,
            (Self::CustomPassword(ch, _, _), Self::CustomPassword(ch2, _, _)) => ch == ch2,
            (_, _) => false,
        }
    }
}

impl fmt::Debug for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Color => write!(f, "InputType::Color"),
            Self::Custom(..) => write!(f, "InputType::Custom"),
            Self::CustomPassword(c, _, _) => write!(f, "InputType::CustomPassword({})", c),
            Self::Email => write!(f, "InputType::Email"),
            Self::Number => write!(f, "InputType::Number"),
            Self::Password(ch) => write!(f, "InputType::Password({})", ch),
            Self::SignedInteger => write!(f, "InputType::SignedInteger"),
            Self::Telephone => write!(f, "InputType::Telephone"),
            Self::Text => write!(f, "InputType::Text"),
            Self::UnsignedInteger => write!(f, "InputType::UnsignedInteger"),
        }
    }
}

impl InputType {
    /// Returns whether the provided char `c` can be pushed to `input` based on `InputType`
    pub fn char_valid(&self, input: &str, c: char) -> bool {
        match self {
            Self::Color => Self::char_valid_for_color(input, c),
            Self::Email => Self::char_valid_for_email(input, c),
            Self::Number => {
                c.is_digit(10) || (['+', '-'].contains(&c) && input.is_empty()) || c == '.'
            }
            Self::Telephone => Self::char_valid_for_phone(input, c),
            Self::SignedInteger => c.is_digit(10) || (['+', '-'].contains(&c) && input.is_empty()),
            Self::UnsignedInteger => c.is_digit(10),
            Self::Password(_) | Self::Text => true,
            Self::Custom(_, char_valid) | Self::CustomPassword(_, _, char_valid) => {
                char_valid(input, c)
            }
        }
    }

    /// Returns whether the entire input is valid based on current input type
    pub fn validate(&self, s: &str) -> bool {
        match self {
            Self::Color => parse_color(s).is_some(),
            Self::Email => parse_email(s).is_some(),
            Self::Number => f64::from_str(s).is_ok(),
            Self::SignedInteger => isize::from_str(s).is_ok(),
            Self::UnsignedInteger => usize::from_str(s).is_ok(),
            Self::Password(_) | Self::Text => true,
            Self::Telephone => parse_phone_number(s).is_some(),
            Self::Custom(validate, _) | Self::CustomPassword(_, validate, _) => validate(s),
        }
    }

    // -- input validate
    fn char_valid_for_email(input: &str, c: char) -> bool {
        c.is_alphanumeric() // Must be alphanumeric
            || [
                '.', '!', '#', '$', '%', '&', '\'', '*', '+', '/', '\\', '=', '?', '^', '_', '`',
                '{', '|', '}', '~', '-',
            ]
            .contains(&c) // Or is symbol
            || (c == '@' && !input.is_empty() && input.find('@').is_none()) // Or is at, but the first in string and string is not empty
    }

    fn char_valid_for_phone(input: &str, c: char) -> bool {
        // Must be digit, or + (but at the begin) or space/- but not empty
        c.is_digit(10)
            || (c == '+' && input.is_empty())
            || ([' ', '-'].contains(&c) && !input.is_empty())
    }

    fn char_valid_for_color(input: &str, c: char) -> bool {
        (c.is_alphanumeric() && !input.starts_with('#'))
            || (input.starts_with('#') && c.is_digit(16))
            || c == ' '
            || (c == '#' && input.is_empty())
            || (c == ',' && input.starts_with("rgb"))
            || (c == '(' && input.starts_with("rgb") && !input.contains('('))
            || (c.is_digit(10) && input.starts_with("rgb"))
            || (c == ')' && input.len() >= 15 && !input.contains(')')) // rgb(xxx,xxx,xxx)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use lazy_regex::{Lazy, Regex};
    use pretty_assertions::assert_eq;

    #[test]
    fn validate_input_type() {
        assert_eq!(InputType::Color.validate("#ff00bb"), true);
        assert_eq!(
            InputType::Email.validate("christian.visintin@github.com"),
            true
        );
        assert_eq!(InputType::Number.validate("-96.4"), true);
        assert_eq!(InputType::Number.validate("-35"), true);
        assert_eq!(InputType::Number.validate("32.4"), true);
        assert_eq!(InputType::SignedInteger.validate("-96"), true);
        assert_eq!(InputType::SignedInteger.validate("+128"), true);
        assert_eq!(InputType::SignedInteger.validate("+128.5"), false);
        assert_eq!(InputType::UnsignedInteger.validate("-96"), false);
        assert_eq!(InputType::UnsignedInteger.validate("+128"), true);
        assert_eq!(InputType::UnsignedInteger.validate("+128.5"), false);
        assert_eq!(InputType::Text.validate("Hello world!"), true);
        assert_eq!(InputType::Password('*').validate("Hello world!"), true);
        assert_eq!(InputType::Telephone.validate("+39 345 777 6117"), true);
        let custom = InputType::Custom(custom_valid, custom_char_valid);
        assert_eq!(custom.validate("v0.7.0"), true);
        assert_eq!(custom.validate("vaaaa"), false);
        let custom = InputType::CustomPassword('*', custom_valid, custom_char_valid);
        assert_eq!(custom.validate("v0.7.0"), true);
        assert_eq!(custom.validate("vaaaa"), false);
    }

    #[test]
    fn validate_input_char_color() {
        // Hex
        assert_eq!(InputType::Color.char_valid("#ff00b", 'b'), true);
        assert_eq!(InputType::Color.char_valid("#ff00b", 'g'), false);
        assert_eq!(InputType::Color.char_valid("#", 'b'), true);
        assert_eq!(InputType::Color.char_valid("", '#'), true);
        assert_eq!(InputType::Color.char_valid("#", '#'), false);
        // Rgb
        assert_eq!(InputType::Color.char_valid("", 'r'), true);
        assert_eq!(InputType::Color.char_valid("r", 'g'), true);
        assert_eq!(InputType::Color.char_valid("rg", 'b'), true);
        assert_eq!(InputType::Color.char_valid("rgb", '('), true);
        assert_eq!(InputType::Color.char_valid("rgb(", '2'), true);
        assert_eq!(InputType::Color.char_valid("rgb(2", '5'), true);
        assert_eq!(InputType::Color.char_valid("rgb(255", ','), true);
        assert_eq!(InputType::Color.char_valid("rgb(255, 255, 255", ')'), true);
        assert_eq!(
            InputType::Color.char_valid("rgb(255, 255, 255)", ')'),
            false
        );
        assert_eq!(InputType::Color.char_valid("rgb(", '('), false);
        assert_eq!(InputType::Color.char_valid("cr", 'i'), true);
        assert_eq!(InputType::Color.char_valid("", 'c'), true);
        assert_eq!(InputType::Color.char_valid("c", '#'), false);
    }

    #[test]
    fn validate_input_email() {
        assert_eq!(InputType::Email.char_valid("chrostaceo", '.'), true);
        assert_eq!(InputType::Email.char_valid("chrostaceo.", 'v'), true);
        assert_eq!(
            InputType::Email.char_valid("chrostaceo.veeseenteen", '1'),
            true
        );
        assert_eq!(
            InputType::Email.char_valid("chrostaceo.veeseenteen1997", '!'),
            true
        );
        assert_eq!(
            InputType::Email.char_valid("chrostaceo.veeseenteen1997!", '@'),
            true
        );
        assert_eq!(
            InputType::Email.char_valid("chrostaceo.veeseenteen1997!@", '@'),
            false
        );
        assert_eq!(
            InputType::Email.char_valid("chrostaceo.veeseenteen1997!@", 'g'),
            true
        );
        assert_eq!(
            InputType::Email.char_valid("chrostaceo.veeseenteen1997!@gmail", '.'),
            true
        );
        assert_eq!(
            InputType::Email.char_valid("chrostaceo.veeseenteen1997!@gmail.co", 'm'),
            true
        );
    }

    #[test]
    fn validate_input_char_phone() {
        assert_eq!(InputType::Telephone.char_valid("", '+'), true);
        assert_eq!(InputType::Telephone.char_valid("+", '+'), false);
        assert_eq!(InputType::Telephone.char_valid("", '-'), false);
        assert_eq!(InputType::Telephone.char_valid("", ' '), false);
        assert_eq!(InputType::Telephone.char_valid("", 'b'), false);
        assert_eq!(InputType::Telephone.char_valid("+", '3'), true);
        assert_eq!(InputType::Telephone.char_valid("+39", ' '), true);
        assert_eq!(InputType::Telephone.char_valid("+39 ", '3'), true);
        assert_eq!(InputType::Telephone.char_valid("+39 345", ' '), true);
        assert_eq!(
            InputType::Telephone.char_valid("+39 345 777 611", '7'),
            true
        );
    }

    #[test]
    fn validate_input_char_numbers() {
        assert_eq!(InputType::Number.char_valid("", '+'), true);
        assert_eq!(InputType::Number.char_valid("", '-'), true);
        assert_eq!(InputType::Number.char_valid("", '.'), true);
        assert_eq!(InputType::Number.char_valid("", '1'), true);
        assert_eq!(InputType::Number.char_valid("+", '1'), true);
        assert_eq!(InputType::Number.char_valid("-", '2'), true);
        assert_eq!(InputType::Number.char_valid("-24", '.'), true);
        assert_eq!(InputType::Number.char_valid("-24.", '5'), true);
        assert_eq!(InputType::Number.char_valid("-", '+'), false);
        assert_eq!(InputType::Number.char_valid("24", '-'), false);
        assert_eq!(InputType::Number.char_valid("24", 'a'), false);
        // Integers
        assert_eq!(InputType::SignedInteger.char_valid("", '+'), true);
        assert_eq!(InputType::SignedInteger.char_valid("", '-'), true);
        assert_eq!(InputType::SignedInteger.char_valid("+2", '2'), true);
        assert_eq!(InputType::SignedInteger.char_valid("", '1'), true);
        assert_eq!(InputType::SignedInteger.char_valid("-52", '-'), false);
        assert_eq!(InputType::SignedInteger.char_valid("-52", 'a'), false);
        // Unsigned integer
        assert_eq!(InputType::UnsignedInteger.char_valid("", '2'), true);
        assert_eq!(InputType::UnsignedInteger.char_valid("", '+'), false);
        assert_eq!(InputType::UnsignedInteger.char_valid("", 'b'), false);
        assert_eq!(InputType::UnsignedInteger.char_valid("", '.'), false);
        assert_eq!(InputType::UnsignedInteger.char_valid("24", '5'), true);
    }

    #[test]
    fn validate_input_char_text() {
        assert_eq!(InputType::Text.char_valid("", 'a'), true);
        assert_eq!(InputType::Password('*').char_valid("", 'b'), true);
    }

    #[test]
    fn validate_input_char_custom() {
        let custom = InputType::Custom(custom_valid, custom_char_valid);
        assert_eq!(custom.char_valid("", 'v'), true);
        assert_eq!(custom.char_valid("v", 'v'), false);
        assert_eq!(custom.char_valid("v", '0'), true);
        assert_eq!(custom.char_valid("v0.7", '.'), true);
        assert_eq!(custom.char_valid("v0.7.", '0'), true);
        let custom = InputType::CustomPassword('*', custom_valid, custom_char_valid);
        assert_eq!(custom.char_valid("", 'v'), true);
        assert_eq!(custom.char_valid("v", 'v'), false);
        assert_eq!(custom.char_valid("v", '0'), true);
        assert_eq!(custom.char_valid("v0.7", '.'), true);
        assert_eq!(custom.char_valid("v0.7.", '0'), true);
    }

    fn custom_valid(s: &str) -> bool {
        static TEST_REGEX: Lazy<Regex> = lazy_regex!(r".*(:?[0-9]\.[0-9]\.[0-9])");
        TEST_REGEX.is_match(s)
    }

    fn custom_char_valid(s: &str, c: char) -> bool {
        s.is_empty() || (!s.is_empty() && (c.is_numeric() || c == '.'))
    }
}
