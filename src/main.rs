use std::sync::Arc;
use std::time::Duration;
use dotenv::dotenv;
use tokio::sync::Mutex;
use tokio::time::Instant;
use crate::models::dto::calculator_dto::{CalculatorDto, Operation};
use crate::models::statistics::Statistics;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = std::env::var("HOST").expect("HOST is invalid!");
    let arc_host = Arc::new(host);

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!();
        println!("Arguments not found! Use --help.");
        println!();
        return Ok(());
    }

    if args[1] == "--help" {
        println!();
        println!("Arguments:");
        println!("\t--count-messages: Send messages to server in multitask mode. Receiving from 1 to 100.");
        println!();
        return Ok(());
    }

    if args[1] != "--count-messages" {
        println!();
        println!("Argument \"count-messages\" not found! Use --help.");
        println!();
        return Ok(());
    }

    if args.len() < 3 {
        println!();
        println!("Not found number!");
        println!();
        return Ok(());
    }

    let count_message = match args[2].parse::<u32>() {
        Ok(o) => o,
        Err(_) => {
            println!();
            println!("Argument \"count-messages\" receiving only number!");
            println!();
            return Ok(());
        },
    };

    if count_message == 0 || count_message > 100 {
        println!();
        println!("Argument \"count-messages\" received number range from 1 to 100!");
        println!();
        return Ok(());
    }

    let stats = Arc::new(Mutex::new(Statistics::default()));

    let mut vec = Vec::new();
    for _ in 0..count_message {
        let clone_stats = Arc::clone(&stats);
        let clone_host = Arc::clone(&arc_host);

        let task = tokio::spawn(async move {
            let start_time = Instant::now();

            let calculator = CalculatorDto {
                first_number: 1.0,
                last_number: 2.0,
                operation: Operation::Div
            };

            let json = serde_json::to_string(&calculator).unwrap();
            let client = reqwest::Client::new();

            let response = client.post(format!("{}/work", clone_host))
                .body(json.clone())
                .timeout(Duration::from_secs(2))
                .send()
                .await;

            let response = match response {
                Ok(o) => o,
                Err(_) => return 1,
            };

            if response.status().is_success() {
                let mut lock = clone_stats.lock().await;
                lock.add_success_response();
                lock.update_time_stats(start_time.elapsed());
                drop(lock);

                return 0;
            }

            1
        });

        vec.push(task);
    }

    let start_session_time = Instant::now();
    for task in vec {
        let result = task.await;

        if result.is_err() || result.unwrap() == 1 {
            stats.lock().await.update_total_time_connection(start_session_time.elapsed());
            stats.lock().await.print_report();
            return Ok(());
        };
    }

    stats.lock().await.update_total_time_connection(start_session_time.elapsed());
    stats.lock().await.print_report();
    Ok(())
}

mod models;