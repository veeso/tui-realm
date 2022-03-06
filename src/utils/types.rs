//! ## Types
//!
//! This module exposes types used by utilities

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

/// ## PhoneNumber
///
/// Represents a phone number
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct PhoneNumber {
    /// Prefix number (without `00` or `+`)
    pub prefix: Option<String>,
    /// Phone number without prefix
    pub number: String,
}

impl PhoneNumber {
    pub fn new<S: AsRef<str>>(prefix: Option<S>, number: S) -> Self {
        let number = number.as_ref().replace(' ', "");
        let number = number.replace('-', "");
        Self {
            prefix: prefix.map(|x| x.as_ref().to_string()),
            number,
        }
    }

    /// ### phone_number
    ///
    /// Returns the full number with syntax `+{prefix}{number}`
    pub fn phone_number(&self) -> String {
        match &self.prefix {
            None => self.number.clone(),
            Some(prefix) => format!("+{}{}", prefix, self.number),
        }
    }
}

/// ## Email
///
/// Represents an email address
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Email {
    /// Address name (e.g. `foo.bar@preema.it` => `foo.bar`)
    pub name: String,
    /// Email agent name (e.g. `foo.bar@preema.it` => `preema.it`)
    pub agent: String,
}

impl Email {
    pub fn new<S: AsRef<str>>(name: S, agent: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            agent: agent.as_ref().to_string(),
        }
    }

    /// ### address
    ///
    /// Returns the email address
    pub fn address(&self) -> String {
        format!("{}@{}", self.name, self.agent)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn phone_number() {
        let phone = PhoneNumber::new(Some("39"), "345 777 6117");
        assert_eq!(phone.prefix.as_deref().unwrap(), "39");
        assert_eq!(phone.number.as_str(), "3457776117");
        assert_eq!(phone.phone_number().as_str(), "+393457776117");
        let phone = PhoneNumber::new(None, "345-777-6117");
        assert!(phone.prefix.is_none());
        assert_eq!(phone.number.as_str(), "3457776117");
        assert_eq!(phone.phone_number().as_str(), "3457776117");
    }

    #[test]
    fn email() {
        let email = Email::new("cvisintin", "youtube.com");
        assert_eq!(email.name.as_str(), "cvisintin");
        assert_eq!(email.agent.as_str(), "youtube.com");
        assert_eq!(email.address().as_str(), "cvisintin@youtube.com");
    }
}
