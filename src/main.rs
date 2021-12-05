use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli{
    #[structopt(short)]
    pattern: String,
    #[structopt(parse(from_os_str),help = "Path shall be passed")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
}
