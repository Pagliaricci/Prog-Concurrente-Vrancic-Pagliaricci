use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
pub struct Stats {
    pub total_exceptions: usize,
    pub files_processed: usize,
    pub per_file: HashMap<String, usize>,
}

#[derive(Clone)]
pub struct SharedStats {
    inner: Arc<RwLock<Stats>>,
}

impl SharedStats {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Stats::default())),
        }
    }

    pub fn add_file_stats(&self, filename: &str, exception_count: usize) {
        let mut stats = self.inner.write().unwrap();
        stats.total_exceptions += exception_count;
        stats.files_processed += 1;
        stats.per_file.insert(filename.to_string(), exception_count);
    }

    pub fn get_summary(&self) -> String {
        let stats = self.inner.read().unwrap();
        let mut summary = format!(
            "Total exceptions: {}\nFiles processed: {}\nPer file: {{",
            stats.total_exceptions, stats.files_processed
        );

        for (file, count) in &stats.per_file {
            summary.push_str(&format!("\"{}\": {}, ", file, count));
        }

        if stats.per_file.len() > 0 {
            summary.truncate(summary.len() - 2); // Remove last comma
        }

        summary.push_str("}");
        summary
    }
}
