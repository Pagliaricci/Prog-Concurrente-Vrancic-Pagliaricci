mod seq;
mod per_file;
mod per_chunk;

use std::path::PathBuf;
use std::io;

pub use seq::search_sequential;
pub use per_file::search_concurrent_files;
pub use per_chunk::search_concurrent_chunks;

#[derive(Debug, Clone, Copy)]
pub enum SearchMode {
    Sequential,
    ConcurrentPerFile,
    ConcurrentPerChunk(usize),
}

pub fn search(pattern: &str, files: &[PathBuf], mode: SearchMode) -> io::Result<Vec<String>> {
    match mode {
        SearchMode::Sequential => search_sequential(pattern, files),
        SearchMode::ConcurrentPerFile => search_concurrent_files(pattern, files),
        SearchMode::ConcurrentPerChunk(chunk_size) => {
            search_concurrent_chunks(pattern, files, chunk_size)
        }
    }
}
