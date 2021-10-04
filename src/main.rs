use tetetris::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
