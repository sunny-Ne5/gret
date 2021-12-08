use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str), help = "Path shall be passed")]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let f = File::open(&args.path)
        .with_context(|| format!("could not open file {}", &args.path.display()))?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let data = &line
            .with_context(|| format!("could not read line from file `{}`", args.path.display()))?;
        gret::check_match(&data, &args.pattern, &mut std::io::stdout());
    }
    Ok(())
}
