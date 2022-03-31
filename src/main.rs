use clap::Parser;
use factordb::Number;

/// Finds a factor to a number using FactorDB (http://factordb.com/)
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Number to find its factor
    number: String,
}

fn main() {
    let args = Args::parse();
    match Number::get(args.number.clone()) {
        Ok(num) => println!("{} = {}", &args.number, num),
        Err(e) => eprintln!("error: {}", e)
    }
}