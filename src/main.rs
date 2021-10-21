use std::process;

use anyhow::Result;
use tetetris::{flags, settings};

fn main() -> Result<()> {
    let f = flags::config_flags()?;
    let s = settings::load()?;

    tetetris::run(f, s)
}
