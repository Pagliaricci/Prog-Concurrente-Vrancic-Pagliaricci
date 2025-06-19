#[derive(Debug, Clone)]
pub struct CrawlRequest {
    pub url: String,
    pub path: Vec<String>,
    pub depth: usize,
}

#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub found: bool,
    pub path: Vec<String>,
    pub new_links: Vec<CrawlRequest>,
    pub links_processed: usize,
}
