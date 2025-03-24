use crate::CliErr::{FileNotFound, MissingFiles, MissingChunkSize, ParseChunkSizeError, MissingMode, MissingPattern, UnknownMode};
use mini_grep::{grep_chunk, grep_conc, grep_seq};
use std::env;
use std::env::Args;
use std::fs::File;
use std::str::FromStr;

enum CliErr {
    MissingMode,
    MissingChunkSize,
    ParseChunkSizeError,
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
        Err(MissingChunkSize) => {
            print_error("No chunk size was passed, even though selected option is 'c-chunk'".as_ref())
        },
        Err(ParseChunkSizeError) => {
            print_error("Could not parse chunk size".as_ref())
        },
    };
}

fn print_error(message: &str) {
    println!("{}Error{}: {}", "\x1B[31m", "\x1B[0m", message)
}

fn run() -> Result<(), CliErr> {
    let mut args = env::args();
    // First one is the command name and is unused
    args.next();

    let mode = args.next().ok_or(MissingMode)?;
    let chunk_size = get_chunk_size(&mut args, &mode)?;

    let pattern = args.next().ok_or(MissingPattern)?;
    let file_names = get_remaining(&mut args)?;

    let files: Vec<File> = open_files(file_names)?;

    match mode.as_str() {
        "seq" => print_all(grep_seq(pattern.clone(), files)),
        "conc" => print_all(grep_conc(&pattern, files)),
        "c-chunk" => print_all(grep_chunk(&pattern, files, chunk_size)),
        _ => Err(UnknownMode(mode.clone())),
    }
}

fn get_chunk_size(args: &mut Args, mode: &String) -> Result<usize, CliErr> {
    if mode == "c-chunk" {
        let chunks_as_str = args.next().ok_or(MissingChunkSize)?;
        usize::from_str(chunks_as_str.as_str())
            .map_err(|_| ParseChunkSizeError)
    } else {
        Ok(0)
    }
}

fn get_remaining(args: &mut Args) -> Result<impl Iterator<Item = String>, CliErr> {
    if let None = args.peekable().peek() {
        return Err(MissingFiles);
    };
    Ok(args)
}

fn open_files(file_names: impl Iterator<Item = String>) -> Result<Vec<File>, CliErr> {
    file_names
        .map(|file_name| File::open(&file_name).map_err(|_| FileNotFound(file_name.clone())))
        .collect()
}

fn print_all(filtered_lines: Vec<String>) -> Result<(), CliErr> {
    filtered_lines.iter().for_each(|line| println!("{}", line));
    Ok(())
}
