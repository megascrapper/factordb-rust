use ansi_term::Colour::Red;
use clap::{ArgGroup, Parser};
use factordb::Number;
use std::process::exit;

/// Finds a factor to a number using FactorDB (http://factordb.com/)
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(group(ArgGroup::new("factors").args(&["print-factors", "print-unique-factors"])))]
struct Args {
    /// Number to find its factor
    number: String,

    /// Print all factors (including repeating ones) on each line
    #[clap(long)]
    print_factors: bool,

    /// Print unique factors on each line
    #[clap(long)]
    print_unique_factors: bool,
}

fn main() {
    let args = Args::parse();
    match Number::get(args.number.clone()) {
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
        Err(e) => {
            eprintln!("{} {}", Red.paint("error:"), e);
            exit(1);
        }
    }
}
