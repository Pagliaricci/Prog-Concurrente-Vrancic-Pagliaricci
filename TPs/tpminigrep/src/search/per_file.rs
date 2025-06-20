use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::thread;
use std::io;

pub fn search_concurrent_files(pattern: &str, files: &[PathBuf]) -> io::Result<Vec<String>> {
    let mut handles = vec![];

    for file in files.to_owned() {
        let pattern = pattern.to_string();

        let handle = thread::spawn(move || {
            let mut results = Vec::new();
            let file_open = File::open(&file).map_err(|e| e.to_string())?;
            let reader = BufReader::new(file_open);
            for (i, line) in reader.lines().enumerate() {
                match line {
                    Ok(content) if content.contains(&pattern) => {
                        results.push(format!("{}:{}:{}", file.display(), i + 1, content));
                    }
                    _ => {}
                }
            }
            Ok::<_, String>(results)
        });

        handles.push(handle);
    }

    let mut all_results = vec![];
    for handle in handles {
        let res = handle.join().unwrap().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        all_results.extend(res);
    }

    Ok(all_results)
}
