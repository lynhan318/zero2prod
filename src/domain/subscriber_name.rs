use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubcriberName(String);

impl SubcriberName {
    pub fn parse(s: String) -> Result<SubcriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_too_long || is_empty_or_whitespace || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
    pub fn inner(self) -> String {
        self.0
    }
    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl AsRef<str> for SubcriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_name_valid_name() {
        let name = "a".repeat(256);
        assert_ok!(SubcriberName::parse(name));
    }

    #[test]
    fn a_name_empty_is_rejected() {
        let name = "".to_string();
        assert_err!(SubcriberName::parse(name));
    }

    #[test]
    fn a_name_too_long_is_reject() {
        let name = "a".repeat(257);
        assert_err!(SubcriberName::parse(name));
    }

    #[test]
    fn a_name_contain_specific_characters_is_reject() {
        for char in &['/', '"', '<', '>', '\\', '{', '}'] {
            assert_err!(SubcriberName::parse(char.to_string()));
        }
    }

    #[test]
    fn a_name_whitespace_is_reject() {
        let name = " ".to_string();
        assert_err!(SubcriberName::parse(name));
    }
}
