use clap::Parser;
use factordb::FactorDbBlockingClient;
use std::{fmt::Display, process::exit};

/// Finds a factor to a number using FactorDB (http://factordb.com/)
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
// #[clap(group(ArgGroup::new("factors").args(&["print-factors", "print-unique-factors"])))]
struct Args {
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
    let args = Args::parse();
    let client = FactorDbBlockingClient::new();
    let number = args.number.clone();
    if args.json {
        match client.get_json(number) {
            Ok(text) => println!("{}", text),
            Err(e) => print_error(e),
        }
    } else {
        match client.get(number) {
            Ok(num) => {
                if args.unique {
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
