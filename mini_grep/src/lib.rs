use std::thread::JoinHandle;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread,
};

pub fn grep_seq(pattern: String, file_names: Vec<String>) -> Vec<String> {
    file_names
        .into_iter()
        .map(|file_name| { filter_lines_from_file(file_name, pattern.clone()) })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn grep_conc(pattern: String, files: Vec<String>) -> Vec<String> {
    let threads = files.into_iter().map(|file| {
        let pattern_clone = pattern.clone();
        thread::spawn(|| {
            filter_lines_from_file(file, pattern_clone)
                .collect::<Vec<_>>()
        })
    });

    threads.map(|t| t.join().unwrap()).flatten().collect::<Vec<_>>()
}

pub fn grep_chunk(pattern: String, file_names: Vec<String>, chunk_size: usize) -> Vec<String> {
    let file_threads = file_names.into_iter().map(|file_name| {
        let pattern_clone = pattern.clone();
        thread::spawn(move || {
            let mut chunk_threads: Vec<JoinHandle<Vec<String>>> = vec![];

            let mut br = BufReader::new(File::open(file_name).unwrap()).lines();

            loop {
                let chunk: Vec<String> = br.by_ref()
                    .take(chunk_size)
                    .map(|line| line.unwrap())
                    .collect::<Vec<_>>();

                if chunk.is_empty() { break; };

                append_chunk_thread(chunk, &mut chunk_threads, pattern_clone.clone());
            }

            chunk_threads
                .into_iter()
                .map(|t| t.join().unwrap())
                .flatten()
        })
    });

    file_threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .flatten()
        .collect()
}

fn filter_lines_from_file(file_name: String, pattern: String) -> impl Iterator<Item = String> {
    BufReader::new(File::open(file_name).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .filter(move |line| line.contains(&pattern))
}

fn append_chunk_thread(
    buffered_lines: Vec<String>,
    chunk_threads: &mut Vec<JoinHandle<Vec<String>>>,
    pattern: String
) {
    let filtered_lines: JoinHandle<Vec<String>> = thread::spawn(move || {
        buffered_lines
            .into_iter()
            .filter(move |line| line.contains(&pattern))
            .collect()
    });

    chunk_threads.push(filtered_lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file() {
        let result = grep_seq(
            "test".to_string(),
            vec!["resources/test1.txt".to_string()],
        );
        assert_eq!(
            result,
            vec![
                "This is a test file".to_string(),
                "It's a file that's for testing".to_string()
            ]
        );
    }

    #[test]
    fn single_file_conc() {
        let result = grep_conc(
            "test".to_string(),
            vec!["resources/test1.txt".to_string()],
        );
        assert_eq!(
            result,
            vec![
                "This is a test file".to_string(),
                "It's a file that's for testing".to_string()
            ]
        );
    }

    #[test]
    fn two_files_conc() {
        let result = grep_conc(
            "thread".to_string(),
            vec![
                "resources/test1.txt".to_string(),
                "resources/bible.txt".to_string(),
            ],
        );
        assert_found_thread_for_both_texts(result);
    }

    fn assert_found_thread_for_both_texts(result: Vec<String>) {
        assert_eq!(
            result,
            vec![
                "We are multithreading!".to_string(),
                "14:23 That I will not take from a thread even to a shoelatchet, and".to_string(),
                "thread, saying, This came out first.".to_string(),
                "38:30 And afterward came out his brother, that had the scarlet thread".to_string(),
                "scarlet thread in the window which thou didst let us down by: and thou"
                    .to_string(),
                "brake the withs, as a thread of tow is broken when it toucheth the".to_string(),
                "arms like a thread.".to_string(),
                "4:3 Thy lips are like a thread of scarlet, and thy speech is comely:".to_string()
            ]
        )
    }

    #[test]
    fn two_files_chunk() {
        let result = grep_chunk(
            "thread".to_string(),
            vec![
                "resources/test1.txt".to_string(),
                "resources/bible.txt".to_string(),
            ],
            10000,
        );
        assert_found_thread_for_both_texts(result);
    }
}
