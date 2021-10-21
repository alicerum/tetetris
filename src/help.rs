//! Logic for help message handling.

use termion::event::Key;

use crate::settings::KeyBindings;

/// Help message for display in the UI.
pub struct Help(Vec<String>);

impl Help {
    /// Get the amount of lines in the help message.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get an iterator over the help message lines.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|line| line.as_str())
    }
}

impl From<&KeyBindings> for Help {
    fn from(kb: &KeyBindings) -> Self {
        Self(vec![
            "HELP:".to_owned(),
            format!("Move left: {}", format_keys(&kb.move_left)),
            format!("Move right: {}", format_keys(&kb.move_right)),
            format!("Move down: {}", format_keys(&kb.move_down)),
            format!("Rotate clockwise: {}", format_keys(&kb.rotate_cw)),
            format!("Rotate counter-clockwise: {}", format_keys(&kb.rotate_ccw)),
            format!("Hard drop: {}", format_keys(&kb.drop)),
            format!("Pause: {}", format_keys(&kb.pause)),
            "Quit: q C-c".to_owned(),
        ])
    }
}

/// Turn a list of keys into a more user-friendly text.
fn format_keys(keys: &[Key]) -> String {
    keys.iter().fold(String::new(), |mut buf, key| {
        if !buf.is_empty() {
            buf.push(' ');
        }

        let value = match key {
            Key::Left => "←",
            Key::Right => "→",
            Key::Up => "↑",
            Key::Down => "↓",
            Key::Char(' ') => "space",
            Key::Char('\n') => "ret",
            Key::Null => "\\0",
            Key::Esc => "esc",
            _ => "",
        };

        if value.is_empty() {
            if let Key::Char(c) = key {
                buf.push(*c);
            } else {
                buf.push_str(&match key {
                    Key::F(i) => format!("F{}", i),
                    Key::Alt(c) => format!("A-{}", c),
                    Key::Ctrl(c) => format!("C-{}", c),
                    _ => "?".to_owned(),
                });
            }
        } else {
            buf.push_str(value);
        }

        buf
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_format_keys() {
        assert_eq!(
            "↑ space F5 A-x C-y q ret \\0 ?",
            format_keys(&[
                Key::Up,
                Key::Char(' '),
                Key::F(5),
                Key::Alt('x'),
                Key::Ctrl('y'),
                Key::Char('q'),
                Key::Char('\n'),
                Key::Null,
                Key::Home
            ])
        );
    }
}
