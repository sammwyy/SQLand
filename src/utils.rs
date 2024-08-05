use crate::client::SQLand;
use colored::*;
use rand::{
    distributions::{Alphanumeric, Uniform},
    prelude::Distribution,
    Rng,
};
use tracing::info;

pub fn random_string(length: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    s
}

pub fn random_number(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(min..max);
    let throw = die.sample(&mut rng);
    throw
}

pub fn random_length_string(min: usize, max: usize) -> String {
    let length = random_number(min, max);
    random_string(length)
}

pub fn initialize_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .without_time()
        .init();
}

pub fn load_payloads(file_content: &str) -> Vec<String> {
    file_content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .map(String::from)
        .collect()
}

pub async fn load_detected_errors(
    file_content: &str,
    sqland: &SQLand,
    apply_filtering: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut detected_errors: Vec<String> = file_content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .map(String::from)
        .collect();

    if apply_filtering {
        println!("Filtering error matching expressions using vanilla request...");
        let vanilla_response = sqland.send("".to_string()).await?;
        let vanilla_body = vanilla_response.text().await?.to_lowercase();
        detected_errors.retain(|error| !vanilla_body.contains(error));
    }

    Ok(detected_errors)
}

pub fn log_settings(args: &crate::cli::Args, offset: u128) {
    info_setting_str("Method", &args.method.to_uppercase());
    info_setting_u128("Offset", offset);
    info_setting_u128("Offset Samples", args.offset_samples);
    info_setting_bool("Filtering", !args.no_filtering);
}

// Log values.
pub fn info_title(text: &str) {
    info!(" {} {}", "*".bold().red(), text.yellow());
}

pub fn info_value_num(key: &str, value: u128) {
    info!(
        "   - {} {}",
        key.bold().cyan(),
        format!("{}", value).bright_magenta()
    );
}

pub fn info_value_str(key: &str, value: &str) {
    info!("   - {} {}", key.bold().cyan(), value);
}

// Log settings.
pub fn info_setting_str(key: &str, value: &str) {
    info!(" > {} {}", key.bold().cyan(), value);
}

pub fn info_setting_u128(key: &str, value: u128) {
    info!(
        " > {} {}",
        key.bold().cyan(),
        format!("{}", value).bright_magenta()
    );
}

pub fn info_setting_bool(key: &str, value: bool) {
    let value_str = if value {
        format!("{}", value).green()
    } else {
        format!("{}", value).red()
    };
    info!(" > {} {}", key.bold().cyan(), value_str);
}
