use crate::CliErr::{FileNotFound, MissingFiles, MissingMode, MissingPattern, UnknownMode};
use mini_grep::{grep_chunk, grep_conc, grep_seq};
use std::env;
use std::fs::File;

enum CliErr {
    MissingMode,
    MissingPattern,
    MissingFiles,
    FileNotFound(String),
    UnknownMode(String),
}

fn main() {
    // TODO: Better error messages. Should display the desired command structure,
    //  explaining each argument
    match run() {
        Ok(()) => {}
        Err(MissingMode) => {
            print_error("No mode was passed. Must be one of 'seq', 'conc' and 'c-chunk.'")
        }
        Err(MissingPattern) => {
            print_error("No pattern was passed. Must be a string to be searched.")
        }
        Err(MissingFiles) => print_error("No file names were passed. Must be at least one."),
        Err(FileNotFound(file_name)) => {
            print_error(format!("Could not find file '{}'.", file_name).as_str())
        }
        Err(UnknownMode(mode)) => print_error(format!("Unknown mode '{}'.", mode).as_str()),
    };
}

fn print_error(message: &str) {
    println!("{}Error{}: {}", "\x1B[31m", "\x1B[0m", message)
}

fn run() -> Result<(), CliErr> {
    let args: Vec<String> = env::args().collect();

    let mode = args.get(1).ok_or(MissingMode)?;
    let pattern = args.get(2).ok_or(MissingPattern)?;
    let file_names = get_remaining(&args)?;

    let files: Vec<File> = open_files(file_names)?;

    match mode.as_str() {
        "seq" => print_all(grep_seq(pattern.clone(), files)),
        "conc" => print_all(grep_conc(&pattern, files)),
        "c-chunk" => print_all(grep_chunk(&pattern, files, 10000)),
        // TODO: chunk size should actually be passed.
        //  If the command is of the form:
        //  cargo run -- c-chunk 5 tell frankenstein.txt romeo_and_juliet.txt
        //  then the code above should actually be updated to iterate args,
        //  instead of accessing positions directly
        _ => Err(UnknownMode(mode.clone())),
    }
}

fn get_remaining(args: &Vec<String>) -> Result<&[String], CliErr> {
    let file_names = &args[3..];

    if file_names.is_empty() {
        return Err(MissingFiles);
    };
    Ok(file_names)
}

fn open_files(file_names: &[String]) -> Result<Vec<File>, CliErr> {
    file_names
        .iter()
        .map(|file_name| File::open(file_name).map_err(|_| FileNotFound(file_name.clone())))
        .collect()
}

fn print_all(filtered_lines: Vec<String>) -> Result<(), CliErr> {
    filtered_lines.iter().for_each(|line| println!("{}", line));
    Ok(())
}
