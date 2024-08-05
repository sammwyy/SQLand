mod cli;
mod client;
mod utils;

use crate::cli::{args_to_settings, Args};
use crate::client::SQLand;
use crate::utils::{
    initialize_tracing_subscriber, load_detected_errors, load_payloads, log_settings,
};

use clap::Parser;
use colored::*;
use std::sync::Arc;
use tracing::info;
use utils::{info_title, info_value_num, info_value_str};

const SQL_DETECTED_ERRORS: &str = include_str!("../data/sql_detected_errors.txt");
const SQL_ERROR_BASED_PAYLOADS: &str = include_str!("../data/sql_error_based_vecs.txt");
const SQL_TIME_BASED_PAYLOADS: &str = include_str!("../data/sql_time_based_vecs.txt");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber
    initialize_tracing_subscriber();

    // Parse command line arguments
    let args = Args::parse();

    // Convert arguments to settings
    let settings = args_to_settings(&args);

    // Create SQLand instance
    let sqland = Arc::new(SQLand::new(args.url.clone(), Some(settings)));

    // Define payloads for SQL Injection testing
    let time_based_payloads = Arc::new(load_payloads(SQL_TIME_BASED_PAYLOADS));
    let error_based_payloads = Arc::new(load_payloads(SQL_ERROR_BASED_PAYLOADS));

    // Calculate timing offset
    let offset_samples = args.offset_samples;
    let offset = if offset_samples > 0 {
        info!("Calculating response latency offset...");
        sqland.calculate_avg_res_time(offset_samples).await.unwrap()
    } else {
        args.offset
    };

    // Get initial vanilla request for error messages
    let detected_errors = Arc::new(
        load_detected_errors(SQL_DETECTED_ERRORS, &sqland, !args.no_filtering)
            .await
            .unwrap(),
    );

    // Start attack
    info!("Starting attack with configuration:");
    log_settings(&args, offset);

    // Perform SQL Injection testing
    let time_based = test_time_based_injection(
        sqland.clone(),
        time_based_payloads.clone(),
        offset,
        args.workers,
    )
    .await;
    let error_based = test_error_based_injection(
        sqland.clone(),
        error_based_payloads.clone(),
        detected_errors.clone(),
        args.workers,
    )
    .await;

    if !error_based && !time_based {
        info!(
            "{}",
            "No vulnerability detected on specified target.".green()
        );
    } else {
        info!(
            "{}",
            "One or more vulnerabilities have been found on the target.".red()
        );
    }

    Ok(())
}

async fn test_time_based_injection(
    sqland: Arc<SQLand>,
    payloads: Arc<Vec<String>>,
    offset: u128,
    workers: usize,
) -> bool {
    info!("Starting Time-based Blind SQL Injection testing...");

    let payloads_len = payloads.len();
    let mut handles = Vec::new();

    let chunk_size = if workers > payloads_len {
        payloads_len
    } else {
        payloads_len / workers
    };

    for chunk in payloads.chunks(chunk_size) {
        let sqland_clone = sqland.clone();
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            for payload in chunk {
                if send_time_based(&sqland_clone, payload, offset)
                    .await
                    .unwrap_or(false)
                {
                    return true;
                }
            }
            false
        });
        handles.push(handle);
    }

    let mut result = false;
    for handle in handles {
        if handle.await.unwrap_or(false) {
            result = true;
        }
    }

    result
}

async fn test_error_based_injection(
    sqland: Arc<SQLand>,
    payloads: Arc<Vec<String>>,
    detected_errors: Arc<Vec<String>>,
    workers: usize,
) -> bool {
    info!("Starting Error-based Blind SQL Injection testing...");

    let payloads_len = payloads.len();
    let mut handles = Vec::new();

    let chunk_size = if workers > payloads_len {
        1
    } else {
        payloads_len / workers
    };

    for chunk in payloads.chunks(chunk_size) {
        let sqland_clone = sqland.clone();
        let detected_errors_clone = Arc::clone(&detected_errors);
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            for payload in chunk {
                if send_error_based(&sqland_clone, payload, &detected_errors_clone)
                    .await
                    .unwrap_or(false)
                {
                    return true;
                }
            }
            false
        });
        handles.push(handle);
    }

    let mut result = false;
    for handle in handles {
        if handle.await.unwrap_or(false) {
            result = true;
        }
    }

    result
}

async fn send_time_based(
    sqland: &SQLand,
    payload: String,
    offset: u128,
) -> Result<bool, Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let _response = sqland.send(payload.clone()).await?;
    let duration = start.elapsed();

    // Check if the response time is significantly higher
    if duration.as_millis() > (5000 + offset) {
        info_title("Potential time-based SQL injection detected.");
        info_value_num("Response time", duration.as_millis());
        info_value_str("Payload", &payload);
        return Ok(true);
    }

    Ok(false)
}

async fn send_error_based(
    sqland: &SQLand,
    payload: String,
    detected_errors: &[String],
) -> Result<bool, Box<dyn std::error::Error>> {
    let response = sqland.send(payload.clone()).await?;
    let status = response.status();
    let body = response.text().await?.to_lowercase();

    // Check for typical SQL error messages or status code 500
    for error in detected_errors {
        if body.contains(error) {
            info_title("Potential error-based SQL injection detected.");
            info_value_str("Error", error);
            info_value_str("Payload", &payload);
            return Ok(true);
        }
    }

    if status.is_server_error() {
        info_title("Potential status-based SQL injection detected.");
        info_value_num("Status code", status.as_u16().into());
        info_value_str("Payload", &payload);
        return Ok(true);
    }

    Ok(false)
}
