use anyhow::Result;
use clap::Parser;

use sudoku_wave;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File to solve
    #[clap(short, long)]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let problem = std::fs::read_to_string(args.file)?;
    sudoku_wave::solve(&problem)?;
    Ok(())
}
