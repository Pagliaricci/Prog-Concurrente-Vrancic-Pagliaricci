use crate::actors::actor_loop;
use crate::messages::{CrawlRequest, CrawlResult};
use std::collections::HashSet;
use tokio::sync::mpsc;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct CrawlOutput {
    pub shortest_path: Option<Vec<String>>,
    pub total_links_processed: usize,
}

pub struct BfsManager {
    actors: usize,
    start: String,
    target: String,
    max_depth: usize,
}

impl BfsManager {
    pub fn new(actors: usize, start: String, target: String, max_depth: usize) -> Self {
        Self {
            actors,
            start,
            target,
            max_depth,
        }
    }

    pub async fn start(self) -> CrawlOutput {
        let client = Client::new();
        let (tx_task, rx_task) = mpsc::channel::<CrawlRequest>(1000);
        let (tx_result, mut rx_result) = mpsc::channel::<CrawlResult>(1000);

        let rx_task = Arc::new(Mutex::new(rx_task));

        for _ in 0..self.actors {
            let rx_task = rx_task.clone();
            let tx_result = tx_result.clone();
            let target = self.target.clone();
            let client = client.clone();

            tokio::spawn(async move {
                actor_loop(rx_task, tx_result, target, client).await;
            });
        }

        let mut visited = HashSet::new();
        let mut total_links_processed = 0;

        let initial = CrawlRequest {
            url: self.start.clone(),
            path: vec![self.start.clone()],
            depth: self.max_depth,
        };

        let _ = tx_task.send(initial).await;

        while let Some(result) = rx_result.recv().await {
            total_links_processed += result.links_processed;

            if result.found {
                return CrawlOutput {
                    shortest_path: Some(result.path),
                    total_links_processed,
                };
            }

            for new_task in result.new_links {
                if visited.insert(new_task.url.clone()) {
                    let _ = tx_task.send(new_task).await;
                }
            }
        }

        CrawlOutput {
            shortest_path: None,
            total_links_processed,
        }
    }
}
