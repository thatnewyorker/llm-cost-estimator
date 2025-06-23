// src/main.rs

use clap::Parser;
use std::error::Error;

/// Estimate the cost of running a local large language model per 1,000 tokens.
#[derive(Parser)]
#[command(name = "llm-cost-estimator", version = "0.1")]
struct Args {
    /// Model variant (e.g., Q8_0)
    #[arg(short, long, value_parser)]
    model: String,

    /// Tokens processed per second
    #[arg(short, long, value_parser)]
    tokens_per_sec: f64,

    /// GPU power consumption in watts
    #[arg(short, long, value_parser)]
    gpu_power_watts: f64,

    /// Cost of electricity in USD/kWh
    #[arg(short, long, value_parser)]
    cost_per_kwh: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Calculate the number of seconds per 1000 tokens
    let secs_per_1k_tokens = 1000.0 / args.tokens_per_sec;

    // Convert GPU power to kWh per second
    let gpu_power_kwh_per_sec = args.gpu_power_watts / (1000.0 * 3600.0);

    // Calculate the cost per 1000 tokens
    let cost_per_1k_tokens = (gpu_power_kwh_per_sec * secs_per_1k_tokens) * args.cost_per_kwh;

    // Tokens processed per kWh
    let tokens_per_kwh = args.tokens_per_sec / gpu_power_kwh_per_sec * 3600.0;

    println!("Cost per 1,000 tokens: ${:.4}", cost_per_1k_tokens);
    println!("Tokens processed per kWh: {:.2}M", tokens_per_kwh / 1e6);
    println!(
        "Estimated cost for 1 million tokens: ${:.2}",
        (cost_per_1k_tokens * 1000.0) / 1.0
    );

    Ok(())
}
