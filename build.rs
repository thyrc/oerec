extern crate clap_complete;

use clap_complete::{generate_to, shells};
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=src/cli.rs");
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let mut app = build_cli();
    generate_to(shells::Bash, &mut app, env!("CARGO_PKG_NAME"), &outdir)?;

    fs::copy(
        Path::new(&outdir).join("oerec.bash"),
        Path::new("shell/oerec.bash"),
    )?;

    Ok(())
}
