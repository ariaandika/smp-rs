use anyhow::Context;
use const_format::formatcp;
use std::{fs, io, os::unix::fs::PermissionsExt, thread};

const DIST: &str = "dist";

const TW_VERSION: &str = "4.0.14";
const TW_BIN: &str = formatcp!("{DIST}/tailwind");
const TW_OS: &str = if cfg!(windows) {
    "tailwindcss-windows-x64.exe"
} else {
    "tailwindcss-linux-x64"
};
const TW_URL: &str = formatcp!(
    "https://github.com/tailwindlabs/tailwindcss/releases/download/v{TW_VERSION}/{TW_OS}"
);

const HX_VERSION: &str = "2.0.4";
const HX_PATH: &str = formatcp!("{DIST}/hx.js");
const HX_URL: &str = formatcp!("https://unpkg.com/htmx.org@{HX_VERSION}/dist/htmx.min.js");

const CR_PATH: &str = formatcp!("{DIST}/carousel.js");
const CR_URL: &str = formatcp!("https://unpkg.com/embla-carousel/embla-carousel.umd.js");

fn main() -> anyhow::Result<()> {
    if !fs::exists(DIST)? {
        fs::create_dir(DIST).context("failed to create `dist`")?;
    }

    let handles = [
        thread::spawn(tailwind),
        thread::spawn(scripts),
    ];

    for handle in handles {
        match handle.join() {
            Ok(Err(err)) => eprintln!("{err}"),
            Ok(Ok(())) => {}
            Err(_) => eprintln!("failed to join handle")
        }
    }

    Ok(())
}

fn tailwind() -> anyhow::Result<()> {
    download(TW_BIN, TW_URL, true).context("failed to download tailwind")?;
    Ok(())
}

fn scripts() -> anyhow::Result<()> {
    download(HX_PATH, HX_URL, false).context("failed to download htmx")?;
    download(CR_PATH, CR_URL, false).context("failed to download carousel")?;
    Ok(())
}

fn download(path: &str, url: &str, is_exec: bool) -> anyhow::Result<()> {
    if fs::exists(path)? {
        return Ok(());
    }

    let mut file = fs::File::create(path).context("failed to create tailwind file")?;

    io::copy(
        &mut ureq::get(url).call()?.into_body().into_reader(),
        &mut file,
    )?;

    #[cfg(unix)]
    if is_exec {
        let mut perm = fs::metadata(path)?.permissions();
        perm.set_mode(0o744);
        file.set_permissions(perm).context("failed to make executable")?;
    }

    Ok(())
}

