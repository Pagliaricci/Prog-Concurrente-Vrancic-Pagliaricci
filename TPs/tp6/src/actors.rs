use crate::messages::{CrawlRequest, CrawlResult};
use crate::wiki::fetch_links;
use tokio::sync::mpsc::{Receiver, Sender};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn actor_loop(
    rx: Arc<Mutex<Receiver<CrawlRequest>>>,
    tx_result: Sender<CrawlResult>,
    target_url: String,
    client: Client,
) {
    loop {
        let task = {
            let mut rx = rx.lock().await;
            rx.recv().await
        };
        let Some(task) = task else { break; };
        if task.depth == 0 {
            continue;
        }

        let mut links_processed = 0;
        let mut new_requests = Vec::new();

        match fetch_links(&client, &task.url).await {
            Ok(links) => {
                links_processed = links.len();

                for link in links {
                    if link == target_url {
                        let mut path = task.path.clone();
                        path.push(link.clone());

                        let _ = tx_result.send(CrawlResult {
                            found: true,
                            path,
                            new_links: vec![],
                            links_processed,
                        }).await;
                        return;
                    }

                    if !task.path.contains(&link) {
                        let mut new_path = task.path.clone();
                        new_path.push(link.clone());
                        new_requests.push(CrawlRequest {
                            url: link,
                            path: new_path,
                            depth: task.depth - 1,
                        });
                    }
                }
            }
            Err(_) => {}
        }

        let _ = tx_result.send(CrawlResult {
            found: false,
            path: vec![],
            new_links: new_requests,
            links_processed,
        }).await;
    }
}
