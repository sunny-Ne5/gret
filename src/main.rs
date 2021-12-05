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

fn main() {
    let args = Cli::from_args();
    let f = File::open(&args.path).expect("couldn't open file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(data) => {
                if data.contains(&args.pattern) {
                    println!("{}", data);
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
