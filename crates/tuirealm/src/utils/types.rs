//! ## Types
//!
//! This module exposes types used by utilities

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
        let number = number.as_ref().replace([' ', '-'], "");
        Self {
            prefix: prefix.map(|x| x.as_ref().to_string()),
            number,
        }
    }

    /// Returns the full number with syntax `+{prefix}{number}`
    pub fn phone_number(&self) -> String {
        match &self.prefix {
            None => self.number.clone(),
            Some(prefix) => format!("+{prefix}{}", self.number),
        }
    }
}

/// Represents an email address
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Email {
    /// Address name (e.g. `foo.bar@preema.it` => `foo.bar`)
    pub name: String,
    /// Email agent name (e.g. `foo.bar@preema.it` => `preema.it`)
    pub agent: String,
}

impl Email {
    pub fn new<S: Into<String>>(name: S, agent: S) -> Self {
        Self {
            name: name.into(),
            agent: agent.into(),
        }
    }

    /// Returns the email address
    pub fn address(&self) -> String {
        format!("{}@{}", self.name, self.agent)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

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
