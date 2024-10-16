mod core;
use core::stock_ticker::functional_test;

use clap::Parser;

/// Simple program to monitor stock tickers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Symbol of a stock
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    functional_test(&args.name);
}