use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

pub fn search_sequential(pattern: &str, files: &[PathBuf]) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    for file in files {
        let file_open = File::open(file)?;
        let reader = BufReader::new(file_open);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if line.contains(pattern) {
                results.push(format!("{}:{}:{}", file.display(), i + 1, line));
            }
        }
    }
    Ok(results)
}
