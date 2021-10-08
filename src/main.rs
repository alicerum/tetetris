use std::process;
use tetetris;
use tetetris::flags;

fn main() {
    let f = match flags::config_flags() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        },
    };

    if let Err(e) = tetetris::run(f) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
