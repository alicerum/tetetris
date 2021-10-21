//! Custom deserializers for settings data.

use serde::{
    de::{Deserializer, Visitor},
    Deserialize,
};
use termion::event::Key;

/// Deserialize a [`Vec<Key>`] from a sequence of strings.
pub fn keys<'de, D>(deserializer: D) -> Result<Vec<Key>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(KeysVisitor)
}

struct KeysVisitor;

impl<'de> Visitor<'de> for KeysVisitor {
    type Value = Vec<Key>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("sequence of keyboard keys")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Wrapper(#[serde(deserialize_with = "key")] termion::event::Key);

        let mut buf = Vec::with_capacity(seq.size_hint().unwrap_or_default());

        while let Some(element) = seq.next_element::<Wrapper>()? {
            buf.push(element.0);
        }

        Ok(buf)
    }
}

/// Deserialize a [`Key`] from a string value.
pub fn key<'de, D>(deserializer: D) -> Result<Key, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(KeyVisitor)
}

struct KeyVisitor;

impl<'de> Visitor<'de> for KeyVisitor {
    type Value = Key;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("keyboard key encoded as string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "Backspace" => Key::Backspace,
            "Left" => Key::Left,
            "Right" => Key::Right,
            "Up" => Key::Up,
            "Down" => Key::Down,
            "Home" => Key::Home,
            "End" => Key::End,
            "PageUp" => Key::PageUp,
            "PageDown" => Key::PageDown,
            "BackTab" => Key::BackTab,
            "Delete" => Key::Delete,
            "Insert" => Key::Insert,
            "Esc" => Key::Esc,
            _ => {
                if let Some(c) = strip_char(v, "A-") {
                    Key::Alt(c)
                } else if let Some(c) = strip_char(v, "C-") {
                    Key::Ctrl(c)
                } else if let Some(c) = v.strip_prefix('F').and_then(|v| v.parse().ok()) {
                    Key::F(c)
                } else if let Some(Some(c)) = (v.len() == 1).then(|| v.chars().next()) {
                    Key::Char(c)
                } else {
                    return Err(E::custom("invalid key binding"));
                }
            }
        })
    }
}

/// Strip a prefix from the given value and return the remainder if only a single char remains.
fn strip_char(value: &str, prefix: &str) -> Option<char> {
    value
        .strip_prefix(prefix)
        .filter(|v| v.len() == 1)
        .and_then(|v| v.chars().next())
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[derive(Debug, PartialEq, Deserialize)]
    struct TestKeys {
        #[serde(deserialize_with = "keys")]
        keys: Vec<Key>,
    }

    #[test]
    fn de_keys() {
        let expect = TestKeys {
            keys: vec![
                Key::Up,
                Key::Alt('x'),
                Key::Ctrl('y'),
                Key::F(5),
                Key::Char('q'),
            ],
        };

        assert_de_tokens(
            &expect,
            &[
                Token::Struct {
                    name: "TestKeys",
                    len: 1,
                },
                Token::Str("keys"),
                Token::Seq { len: Some(5) },
                Token::Str("Up"),
                Token::Str("A-x"),
                Token::Str("C-y"),
                Token::Str("F5"),
                Token::Str("q"),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn de_keys_invalid() {
        assert_de_tokens_error::<TestKeys>(
            &[
                Token::Struct {
                    name: "TestKeys",
                    len: 1,
                },
                Token::Str("keys"),
                Token::Seq { len: Some(1) },
                Token::Str("Null"),
            ],
            "invalid key binding",
        );
    }
}
