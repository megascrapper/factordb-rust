use ansi_term::Colour::Red;
use clap::Parser;
use factordb::FactorDbBlockingClient;
use std::{env::args, fmt::Display, process::exit};

/// Finds a factor to a number using FactorDB (http://factordb.com/)
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
// #[clap(group(ArgGroup::new("factors").args(&["print-factors", "print-unique-factors"])))]
struct Args {
    /// Number to find its factor
    number: String,

    /// Print all factors (including repeating ones) on each line
    #[clap(long)]
    print_factors: bool,

    /// Print unique factors on each line
    #[clap(long)]
    print_unique_factors: bool,

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
                if args.print_factors {
                    for f in num.factor_list() {
                        println!("{}", f);
                    }
                } else if args.print_unique_factors {
                    for f in num.unique_factors() {
                        println!("{}", f);
                    }
                } else {
                    println!("{} = {}", &args.number, num)
                }
            }
            Err(e) => print_error(e),
        }
    }
}
