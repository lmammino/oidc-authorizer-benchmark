use clap::Parser;
use reqwest::Url;
use stats::Stats;
use std::sync::Arc;
use tokengen::TokenGen;
use tokio::sync::RwLock;
mod stats;
mod tokengen;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10000")]
    total_requests: usize,
    #[arg(short, long, default_value = "100")]
    requests_per_sec: usize,
    #[arg(short = 'u', long)]
    target_url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let target_url = Arc::new(Url::parse(&args.target_url).expect("Invalid target URL"));
    let client = Arc::new(reqwest::Client::new());
    let token_gen = TokenGen::default();
    let stats = Arc::new(RwLock::new(Stats::default()));

    while stats.read().await.sent_requests() < args.total_requests {
        let loop_start = tokio::time::Instant::now();
        let curr_requests_sent = stats.read().await.sent_requests();
        let num_requests_in_batch =
            (args.total_requests - curr_requests_sent).min(args.requests_per_sec);

        for _ in 0..num_requests_in_batch {
            tokio::spawn({
                let token = token_gen.next_token();
                let client = client.clone();
                let target_url = target_url.clone();
                let stats = stats.clone();
                async move {
                    let request = client
                        .get(target_url.to_string())
                        .header("Authorization", format!("Bearer {}", token))
                        .send();
                    stats.write().await.inc_requests_sent();
                    let response = request.await.unwrap();
                    let status_code = response.status().as_u16();
                    response.bytes().await.unwrap(); // consumes the response body
                    stats.write().await.inc_status_code(status_code);
                    stats.write().await.inc_completed_requests();
                }
            });
        }

        println!("{}", stats.read().await);

        // waits the remainder of 1second since the last loop iteration before starting a new one
        let loop_end = tokio::time::Instant::now();
        let loop_duration = loop_end - loop_start;
        let sleep_duration = (1000 - loop_duration.as_millis()).max(0) as u64;
        tokio::time::sleep(tokio::time::Duration::from_millis(sleep_duration)).await;
    }

    println!("{}", stats.read().await);
}
