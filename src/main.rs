use anyhow::Result;
use clap::Parser;
use orb_unpack::unpack_from_file;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Orb yaml file to unpack
    pub orb_file: PathBuf,
    /// Directory where the new orb files will be unpacked
    pub destination: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    unpack_from_file(args.orb_file, args.destination)
}
