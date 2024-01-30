use clap::Parser;
use factordb::FactorDbBlockingClient;
use human_panic::setup_panic;
use std::{fmt::Display, process::exit};

/// Finds a factor to a number using FactorDB (http://factordb.com/)
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Number to find its factor
    number: String,

    /// Print unique factors on each line
    #[clap(long)]
    unique: bool,

    /// Print JSON output of FactorDB API
    #[clap(long)]
    json: bool,
}

fn print_error<T: Display>(msg: T) -> ! {
    let argv = std::env::args().collect::<Vec<_>>();
    let app_name = &argv[0];
    eprintln!("error: {}: {}", app_name, msg);
    exit(1)
}

fn main() {
    env_logger::init();
    setup_panic!();
    let cli = Cli::parse();
    let number = cli.number.clone();

    let client = FactorDbBlockingClient::new();
    if cli.json {
        match client.get_json(number) {
            Ok(text) => println!("{}", text),
            Err(e) => print_error(e),
        }
    } else {
        match client.get(number) {
            Ok(num) => {
                if cli.unique {
                    println!(
                        "{}",
                        num.into_unique_factors()
                            .iter()
                            .map(|f| f.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                } else {
                    println!("{}", num)
                }
            }
            Err(e) => print_error(e),
        }
    }
}
