use clap::{ArgAction, Parser};

use crate::client::{SQLandBodyType, SQLandSettings};

#[derive(Parser, Debug)]
#[command(name = "sqland")]
#[command(about = "Automatize SQL Injection vulnerabilities detection", long_about = None)]
pub struct Args {
    /// HTTP method
    #[arg(short = 'x', long, default_value = "get")]
    pub method: String,

    /// Headers
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Cookies
    #[arg(short = 'c', long = "cookie")]
    pub cookies: Vec<String>,

    /// Parameters to fuzz
    #[arg(short = 'p', long = "param")]
    pub params: Vec<String>,

    /// Dummy data for static parameters
    #[arg(short = 'd', long)]
    pub data: Vec<String>,

    /// JSON body
    #[arg(short = 'j', long, action = ArgAction::SetTrue)]
    pub json: bool,

    /// Form body
    #[arg(short = 'f', long, action = ArgAction::SetTrue)]
    pub form: bool,

    /// Time based attack latency offset (For normalization purposes)
    #[arg(short = 'o', long, default_value_t = 0)]
    pub offset: u128,

    /// Samples to calculate avg response time (For normalization purposes)
    #[arg(short = 's', long, default_value_t = 0)]
    pub offset_samples: u128,

    /// Do not use vanilla request for error filtering
    #[arg(short = 'n', long, action = ArgAction::SetTrue)]
    pub no_filtering: bool,

    // Number of simultaneous payload requests
    #[arg(short = 'w', long, default_value_t = 4)]
    pub workers: usize,

    /// The URL to test
    pub url: String,
}

pub fn args_to_settings(args: &Args) -> SQLandSettings {
    let mut settings = SQLandSettings {
        body_type: None,
        cookies: None,
        data: None,
        headers: None,
        method: None,
        params: None,
    };

    if args.json {
        settings.body_type = Some(SQLandBodyType::JSON);
    } else if args.form {
        settings.body_type = Some(SQLandBodyType::FORM);
    }

    if !args.cookies.is_empty() {
        settings.cookies = Some(args.cookies.clone());
    }

    if !args.data.is_empty() {
        settings.data = Some(args.data.clone());
    }

    if !args.headers.is_empty() {
        settings.headers = Some(args.headers.clone());
    }

    if !args.headers.is_empty() {
        settings.headers = Some(args.headers.clone());
    }

    if !args.method.is_empty() {
        settings.method = Some(args.method.clone());
    }

    if !args.params.is_empty() {
        settings.params = Some(args.params.clone());
    }

    return settings;
}
