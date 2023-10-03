use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t = false)]
    no_new_line: bool,

    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut to_print = String::from("");
    if let Some(input) = args.input {
        to_print = input;
    }

    if args.no_new_line {
        print!("{}", to_print);
    } else {
        println!("{}", to_print);
    }
}
