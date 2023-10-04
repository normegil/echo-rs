use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flag to disable new line character at the end of echoed input
    #[arg(short, default_value_t = false)]
    no_new_line: bool,

    /// Flag to enable interpretation of escape characters
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

#[cfg(test)]
mod tests {
    use clap::Parser;
    use super::Args;

    #[test]
    fn args_raw() {
        let args = Args::parse_from(&["echo-rs", "test"]);
        assert_eq!(args.no_new_line, false);
        assert_eq!(args.escapes, false);
        assert_eq!(args.input, Some(String::from("test")));
    }
    #[test]
    fn args_no_new_line() {
        let args = Args::parse_from(&["echo-rs", "-n", "test"]);
        assert_eq!(args.no_new_line, true);
        assert_eq!(args.escapes, false);
        assert_eq!(args.input, Some(String::from("test")));
    }

    #[test]
    fn args_escapes() {
        let args = Args::parse_from(&["echo-rs", "-e", "test"]);
        assert_eq!(args.no_new_line, false);
        assert_eq!(args.escapes, true);
        assert_eq!(args.input, Some(String::from("test")));
    }

    #[test]
    fn args_escapes_no_new_line() {
        let args = Args::parse_from(&["echo-rs", "-e", "-n", "test"]);
        assert_eq!(args.no_new_line, true);
        assert_eq!(args.escapes, true);
        assert_eq!(args.input, Some(String::from("test")));
    }
}