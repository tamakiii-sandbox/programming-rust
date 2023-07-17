use std::env;
use std::fs;
use text_colorizer::*;

// enable println! macro to output {:?}
#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(2);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Err".red().bold(),
                args.filename,
                e
            );
            std::process::exit(3);
        }
    };

    println!("{:?}", args);
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );

    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error: ".red().bold(),
            args.len()
        );

        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}