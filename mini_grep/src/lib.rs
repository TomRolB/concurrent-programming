use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread,
};
use std::thread::ScopedJoinHandle;

pub fn grep_seq(pattern: String, files: Vec<File>) -> Vec<String> {
    files
        .iter()
        .map(|file| {
            BufReader::new(file)
                .lines()
                .map(|line| line.unwrap())
                .filter(|line| line.contains(&pattern))
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn grep_conc(pattern: &String, files: Vec<File>) -> Vec<String> {
    thread::scope(|s| {
        let threads = files.iter().map(|file| {
            s.spawn(move || {
                BufReader::new(file)
                    .lines()
                    .map(|line| line.unwrap())
                    .filter(move |line| line.contains(pattern))
            })
        });

        threads.map(|t| t.join().unwrap()).flatten().collect()
    })
}

pub fn grep_chunk(pattern: &String, files: Vec<File>, chunk_size: usize) -> Vec<String> {
    thread::scope(|s| {
        let file_threads = files.iter().map(|file| {
            s.spawn(move || {

                let mut count = 0;
                let mut buffered_lines: Vec<String> = Vec::with_capacity(chunk_size);
                let mut chunk_threads: Vec<ScopedJoinHandle<'_, Vec<String>>> = vec![];

                let mut br = BufReader::new(file).lines();
                while let Some(line) = br.next() {
                    buffered_lines.push(line.unwrap());
                    count += 1;

                    if count >= chunk_size {
                        let filtered_lines = s.spawn(move || {
                          buffered_lines.into_iter()
                              .filter(move |line| line.contains(pattern))
                              .collect()
                        });

                        chunk_threads.push(filtered_lines);

                        count = 0;
                        buffered_lines = vec![];
                    }
                }

                if buffered_lines.len() > 0 {
                    let filtered_lines = s.spawn(move || {
                        buffered_lines.into_iter()
                            .filter(move |line| line.contains(pattern))
                            .collect()
                    });

                    chunk_threads.push(filtered_lines);
                }

                chunk_threads.into_iter().map(|t| t.join().unwrap()).flatten()
            })
        });

        file_threads.into_iter()
            .map(|t| t.join().unwrap())
            .flatten()
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file() {
        let result = grep_seq(
            "test".to_string(),
            vec![File::open("resources/test1.txt").unwrap()],
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
            &"test".to_string(),
            vec![File::open("resources/test1.txt").unwrap()],
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
            &"thread".to_string(),
            vec![
                File::open("resources/test1.txt").unwrap(),
                File::open("resources/bible.txt").unwrap(),
            ],
        );
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
        );
    }

    #[test]
    fn two_files_chunk() {
        let result = grep_chunk(
            &"thread".to_string(),
            vec![
                File::open("resources/test1.txt").unwrap(),
                File::open("resources/bible.txt").unwrap(),
            ],
            10000
        );
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
        );
    }
}
