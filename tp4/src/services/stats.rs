use std::collections::HashMap;

pub fn get_stats(stats: HashMap<String, usize>) -> String {
    let total_exceptions = stats.values().fold(0, |a, b| a + b);
    let files_processed = stats.len();
    format!("Total exceptions: {}\nFiles processed: {}\nPer file: {:?}", total_exceptions, files_processed, stats)
}
