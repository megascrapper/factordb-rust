use std::process::exit;
use clap::Parser;
use ansi_term::Colour::Red;
use factordb::Number;

/// Finds a factor to a number using FactorDB (http://factordb.com/)
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Number to find its factor
    number: String,

    /// Print all factors (including repeating ones) on each line
    #[clap(long)]
    print_factors: bool,
}

fn main() {
    let args = Args::parse();
    match Number::get(args.number.clone()) {
        Ok(num) => {
            if !args.print_factors {
                println!("{} = {}", &args.number, num)
            } else {
                for f in num.factor_list() {
                    println!("{}", f);
                }
            }
        },
        Err(e) => {
            eprintln!("{} {}", Red.paint("error:"), e);
            exit(1)
        }
    }
}