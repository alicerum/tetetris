//! Management of application settings.

use std::{collections::HashSet, fs, io::ErrorKind};

use anyhow::{ensure, Context, Result};
use directories_next::ProjectDirs;
use serde::Deserialize;
use termion::event::Key;

mod de;

/// Global application settings.
#[derive(Default, Deserialize)]
pub struct Settings {
    pub key_bindings: KeyBindings,
}

/// Custom key-bindings that allow the user to change the keys bound for each game input.
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
#[derive(Deserialize)]
pub struct KeyBindings {
    #[serde(default, deserialize_with = "de::keys")]
    pub move_left: Vec<Key>,
    #[serde(default, deserialize_with = "de::keys")]
    pub move_right: Vec<Key>,
    #[serde(default, deserialize_with = "de::keys")]
    pub move_down: Vec<Key>,
    #[serde(default, deserialize_with = "de::keys")]
    pub rotate_cw: Vec<Key>,
    #[serde(default, deserialize_with = "de::keys")]
    pub rotate_ccw: Vec<Key>,
    #[serde(default, deserialize_with = "de::keys")]
    pub drop: Vec<Key>,
    #[serde(default, deserialize_with = "de::keys")]
    pub pause: Vec<Key>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_left: vec![Key::Left],
            move_right: vec![Key::Right],
            move_down: vec![Key::Down],
            rotate_cw: vec![Key::Char(' '), Key::Char('x'), Key::Up],
            rotate_ccw: vec![Key::Char('z')],
            drop: vec![Key::Char('\n')],
            pause: vec![Key::Esc],
        }
    }
}

impl KeyBindings {
    /// Make sure that each key-binding that's not set by the user is filled with default bindings.
    ///
    /// This allows to have only partial customized bindings defined in the settings file, falling
    /// back to defaults for any command that is not set explicitly.
    fn fill_empty(&mut self) {
        let default = Self::default();
        let fill = |value: &mut Vec<Key>, def: Vec<Key>| {
            if value.is_empty() {
                *value = def;
            }
        };

        fill(&mut self.move_left, default.move_left);
        fill(&mut self.move_right, default.move_right);
        fill(&mut self.move_down, default.move_down);
        fill(&mut self.rotate_cw, default.rotate_cw);
        fill(&mut self.rotate_ccw, default.rotate_ccw);
        fill(&mut self.drop, default.drop);
        fill(&mut self.pause, default.pause);
    }

    /// Make sure all bindings have acceptable values.
    ///
    /// The rules are:
    /// - Each key must only be bound a single time.
    /// - The exit keys `q` and `C-c` are reserved and can't be re-bound.
    fn validate(&self) -> Result<()> {
        let iter = self
            .move_left
            .iter()
            .chain(self.move_right.iter())
            .chain(self.move_down.iter())
            .chain(self.rotate_cw.iter())
            .chain(self.rotate_ccw.iter())
            .chain(self.drop.iter())
            .chain(self.pause.iter());
        let set = iter.clone().collect::<HashSet<_>>();

        ensure!(
            iter.count() == set.len(),
            "key bindings contain duplicate keys"
        );

        ensure!(
            !set.contains(&Key::Char('q')) && !set.contains(&Key::Ctrl('c')),
            "exit keys `q` and `C-c` can't be changed"
        );

        Ok(())
    }
}

/// Load application settings from a commonly known, platform-dependent location.
///
/// The locations for popular OSs are:
/// - Linux: `$XDC_CONFIG_HOME/tetetris/config.toml` or `$HOME/.config/tetetris/config.toml`.
/// - MacOS: `$HOME/Library/Application Support/org.wyvie.Alice-Rum.TeTetris/config.toml`.
/// - Windows: `{RoamingAppData}\Alice Rum\TeTetris\config\config.toml`.
pub fn load() -> Result<Settings> {
    let path = ProjectDirs::from("org.wyvie", "Alice Rum", "TeTetris")
        .context("failed finding project dirs")?
        .config_dir()
        .join("config.toml");

    let buf = match fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Settings::default()),
        Err(e) => return Err(e.into()),
    };

    let mut settings = toml::from_slice::<Settings>(&buf)?;
    settings.key_bindings.fill_empty();
    settings.key_bindings.validate()?;

    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_empty_default() {
        let mut kb = KeyBindings {
            move_left: vec![],
            move_right: vec![],
            move_down: vec![],
            rotate_cw: vec![],
            rotate_ccw: vec![],
            drop: vec![],
            pause: vec![],
        };
        kb.fill_empty();

        assert_eq!(KeyBindings::default(), kb);
    }

    #[test]
    fn validate_default() {
        assert!(KeyBindings::default().validate().is_ok());
    }

    #[test]
    fn validate_unique() {
        let kb = KeyBindings {
            move_left: vec![Key::Right],
            ..KeyBindings::default()
        };

        assert_eq!(
            "key bindings contain duplicate keys",
            kb.validate().unwrap_err().to_string()
        );
    }

    #[test]
    fn validate_reserved() {
        let kb = KeyBindings {
            move_left: vec![Key::Char('q')],
            ..KeyBindings::default()
        };

        assert_eq!(
            "exit keys `q` and `C-c` can't be changed",
            kb.validate().unwrap_err().to_string()
        );
    }
}
