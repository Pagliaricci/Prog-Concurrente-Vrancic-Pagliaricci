use std::fs;
    use std::io::{self, BufRead};
    use std::path::{Path, PathBuf};
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    pub enum SearchMode {
        Sequential,
        ConcurrentPerFile,
        ConcurrentPerChunk(usize),
    }

    pub fn search(pattern: &str, files: &[PathBuf], mode: SearchMode) -> io::Result<Vec<String>> {
        match mode {
            SearchMode::Sequential => search_sequential(pattern, files),
            SearchMode::ConcurrentPerFile => search_concurrent_per_file(pattern, files),
            SearchMode::ConcurrentPerChunk(chunk_size) => search_concurrent_per_chunk(pattern, files, chunk_size),
        }
    }

    fn search_sequential(pattern: &str, files: &[PathBuf]) -> io::Result<Vec<String>> {
        let mut results = Vec::new();
        for file in files {
            results.extend(search_in_file(pattern, file)?);
        }
        Ok(results)
    }

    fn search_concurrent_per_file(pattern: &str, files: &[PathBuf]) -> io::Result<Vec<String>> {
        let (tx, rx) = mpsc::channel();
        for file in files {
            let tx = tx.clone();
            let pattern = pattern.to_string();
            let file = file.clone();
            thread::spawn(move || {
                let result = search_in_file(&pattern, &file);
                tx.send(result).expect("Could not send data!");
            });
        }

        drop(tx); // Close the channel

        let mut results = Vec::new();
        for result in rx {
            results.extend(result?);
        }
        Ok(results)
    }

    fn search_concurrent_per_chunk(pattern: &str, files: &[PathBuf], chunk_size: usize) -> io::Result<Vec<String>> {
        let (tx, rx) = mpsc::channel();
        for file in files {
            let tx = tx.clone();
            let pattern = pattern.to_string();
            let file = file.clone();
            thread::spawn(move || {
                let result = search_in_file_concurrent_chunks(&pattern, &file, chunk_size);
                tx.send(result).expect("Could not send data!");
            });
        }

        drop(tx); // Close the channel

        let mut results = Vec::new();
        for result in rx {
            results.extend(result?);
        }
        Ok(results)
    }

    fn search_in_file(pattern: &str, file: &Path) -> io::Result<Vec<String>> {
        let file = fs::File::open(file)?;
        let reader = io::BufReader::new(file);
        let mut results = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.contains(pattern) {
                results.push(line);
            }
        }

        Ok(results)
    }

    fn search_in_file_concurrent_chunks(pattern: &str, file: &Path, chunk_size: usize) -> io::Result<Vec<String>> {
        let file = fs::File::open(file)?;
        let reader = io::BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
        let lines = Arc::new(lines);
        let (tx, rx) = mpsc::channel();

        for chunk in lines.chunks(chunk_size) {
            let tx = tx.clone();
            let pattern = pattern.to_string();
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                let results: Vec<_> = chunk.into_iter()
                    .filter(|line| line.contains(&pattern))
                    .collect();
                tx.send(results).expect("Could not send data!");
            });
        }

        drop(tx); // Close the channel

        let mut results = Vec::new();
        for result in rx {
            results.extend(result);
        }
        Ok(results)
    }