use crate::CliErr::{MissingFiles, MissingMode, MissingPattern, UnknownMode};
use mini_grep::{grep_chunk, grep_conc, grep_seq};
use std::env;
use std::env::Args;
use std::time::Instant;

enum CliErr {
    MissingMode,
    MissingPattern,
    MissingFiles,
    UnknownMode(String),
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(MissingMode) => {
            print_error("No mode was passed. Must be one of 'seq', 'conc' or 'c-chunk.'")
        }
        Err(MissingPattern) => {
            print_error("No pattern was passed. Must be a string to be searched.")
        }
        Err(MissingFiles) => print_error("No file names were passed. Must be at least one."),
        Err(UnknownMode(mode)) => print_error(format!("Unknown mode '{}'.", mode).as_str()),
    };
}

fn print_error(message: &str) {
    println!(
        "{}Error{}: {}
        \nCommand should be:
        cargo run -- <mode> <pattern> <file 1> <file 2> ... <file n>
        \nWhere:
        * 'mode' must be one of 'seq', 'conc' or 'c-chunk'
        * 'pattern' is a string to be searched'
        * '<file 1> <file 2> ... <file n>' are the paths to the files where the pattern will be searched
        ",
        "\x1B[31m",
        "\x1B[0m",
         message
    );
}

fn run() -> Result<(), CliErr> {
    let mut args = env::args();
    // First one is the command name and is unused
    args.next();

    let mode = args.next().ok_or(MissingMode)?;
    let pattern = args.next().ok_or(MissingPattern)?;
    let file_names: Vec<String> = get_remaining(&mut args)?;

    let starting_time = Instant::now();

    let result = match mode.as_str() {
        "seq" => grep_seq(pattern.clone(), file_names),
        "conc" => grep_conc(pattern.clone(), file_names),
        "c-chunk" => grep_chunk(pattern.clone(), file_names),
        _ => Err(UnknownMode(mode.clone()))?,
    };

    print_all(result, starting_time)
}

fn get_remaining(args: &mut Args) -> Result<Vec<String>, CliErr> {
    let file_names = args.collect::<Vec<String>>();

    if file_names.is_empty() {
        return Err(MissingFiles);
    }
    Ok(file_names)
}

fn print_all(filtered_lines: Vec<String>, starting_time: Instant) -> Result<(), CliErr> {
    let elapsed_time = starting_time.elapsed().as_millis();

    filtered_lines.iter().for_each(|line| println!("{}", line));
    println!(
        "\n(Found {} matches in {}ms)",
        filtered_lines.len(),
        elapsed_time
    );
    Ok(())
}
