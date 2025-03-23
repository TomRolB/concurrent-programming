use std::env;
use std::fs::File;
use mini_grep::grep_seq;
use crate::CliErr::{FileNotFound, MissingFiles, MissingMode, MissingPattern, UnknownMode};

enum CliErr {
    MissingMode,
    MissingPattern,
    MissingFiles,
    FileNotFound(String),
    UnknownMode(String)
}

fn main() {
    // TODO: Better error messages. Should display the desired command structure,
    //  explaining each argument
    match run() {
        Ok(()) => {},
        MissingMode => println!("No mode was passed. Must be one of 'seq', 'conc' and 'c-chunk.'"),
        MissingPattern => println!("No pattern was passed. Must be a string to be searched."),
        MissingFiles => println!("No file names were passed. Must be at least one."),
        FileNotFound(file_nane) => println!("Could not find file '{}'.", file_nane),
    };
}

fn run() -> Result<(), CliErr> {
    let args: Vec<String> = env::args().collect();

    let mode = args.get(1).ok_or(MissingMode)?;
    let pattern = args.get(2).ok_or(MissingPattern)?;
    let file_names = get_remaining(args)?;

    let files: Vec<File> = open_files(file_names)?;

    match mode.as_str() {
        "seq" => print_all(grep_seq(pattern.clone(), files)),
        "conc" => print_all(grep_seq(pattern.clone(), files)),
        "c-chunk" => print_all(grep_seq(pattern.clone(), files)),
        _ => Err(UnknownMode(mode.clone()))
    }
}

fn get_remaining<'a>(args: Vec<String>) -> Result<&'a [String], MissingFiles> {
    let file_names = &args[3..];

    if file_names.is_empty() { return Err(MissingFiles) };
    Ok(file_names)
}

fn open_files(file_names: &[String]) -> Result<Vec<File>, FileNotFound> {
    file_names.iter()
        .map(|file_name| {
            File::open(file_name)
                .map_err(|err| FileNotFound(file_name.clone()))
        })
        .collect()
}

fn print_all(filtered_lines: Vec<String>) -> Ok(()) {
    filtered_lines.iter().for_each(|line| println!("{}", line));
    Ok(())
}
