use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t = false)]
    no_new_line: bool,

    #[arg(short, default_value_t = false)]
    escapes: bool,

    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut to_print = String::from("");
    if let Some(input) = args.input {
        to_print = input;
    }

    if args.escapes {
        to_print = to_print.replace("\\\\", "\\")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");
    } else {
        to_print = to_print.replace("\\", "\\\\");
    }

    if args.no_new_line {
        print!("{}", to_print);
    } else {
        println!("{}", to_print);
    }
}
