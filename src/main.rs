use std::{io, process};

fn main() {
    dotenvy::dotenv().ok();
    if let Err(err) = app() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn app() -> io::Result<()> {
    todo!()
}

