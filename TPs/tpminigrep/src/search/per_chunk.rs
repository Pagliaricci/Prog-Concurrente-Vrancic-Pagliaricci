use std::fs;
use std::path::PathBuf;
use std::thread;
use std::io;

pub fn search_concurrent_chunks(pattern: &str, files: &[PathBuf], chunk_size: usize) -> io::Result<Vec<String>> {
    let mut handles = vec![];

    for file in files {
        let pattern = pattern.to_string();
        let path = file.clone();

        let content = fs::read_to_string(file)?;
        let lines: Vec<_> = content.lines().map(|s| s.to_string()).collect();

        for (chunk_index, chunk) in lines.chunks(chunk_size).enumerate() {
            let chunk = chunk.to_owned();
            let path = path.clone();
            let pattern = pattern.clone();

            let handle = thread::spawn(move || {
                let mut results = vec![];
                for (i, line) in chunk.iter().enumerate() {
                    if line.contains(&pattern) {
                        let global_line_number = chunk_index * chunk_size + i + 1;
                        results.push(format!("{}:{}:{}", path.display(), global_line_number, line));
                    }
                }
                results
            });

            handles.push(handle);
        }
    }

    let mut results = vec![];
    for handle in handles {
        results.extend(handle.join().unwrap());
    }

    Ok(results)
}
