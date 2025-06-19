use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tp6::bfs_manager::BfsManager;
use tp6::wiki::url_to_title;

#[derive(Parser, Debug)]
#[command(name = "Wikipedia Crawler")]
struct Args {
    #[arg(long)]
    actors: Option<usize>,
    #[arg(long)]
    start: String,
    #[arg(long)]
    target: String,
    #[arg(long)]
    max_depth: Option<usize>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let result = BfsManager::new(
        args.actors.unwrap_or_else(num_cpus::get),
        args.start.clone(),
        args.target.clone(),
        args.max_depth.unwrap_or(10),
    )
        .start()
        .await;

    if let Some(path) = &result.shortest_path {
        println!("Shortest path found ({} steps):", path.len());
        for (i, url) in path.iter().enumerate() {
            println!("{}. {}", i + 1, url_to_title(url));
        }
    } else {
        println!("No path found.");
    }
    println!("Total links processed: {}", result.total_links_processed);

    let output_path = "output.json";
    let mut results = if Path::new(output_path).exists() {
        let content = std::fs::read_to_string(output_path).unwrap_or_default();
        serde_json::from_str::<Vec<tp6::bfs_manager::CrawlOutput>>(&content).unwrap_or_default()
    } else {
        Vec::new()
    };
    results.push(result);
    let json = serde_json::to_string_pretty(&results).unwrap();
    let mut file = File::create(output_path).expect("No se pudo crear output.json");
    file.write_all(json.as_bytes()).unwrap();

    println!("Resultado guardado en output.json");
}
