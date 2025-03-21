use std::{fs::File, io::{BufRead, BufReader}};

pub fn grep_seq(pattern: String, files: Vec<File>) -> Vec<String> {
    files.iter()
        .map(|file| BufReader::new(file))
        .map(|buf_reader| buf_reader.lines()
            .map(|line| line.unwrap())
            .filter(|line| line.contains(&pattern)))
        .flatten()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file() {
        let result = grep_seq("test".to_string(), vec![File::open("resources/test1.txt").unwrap()]);
        assert_eq!(result, vec!["This is a test file".to_string(), "It's a file that's for testing".to_string()]);
    }
}
