use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::Instant;

#[derive(Parser)]
#[command(about = "CLI test")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Calc {
        number: u64,
        #[arg(short, long)]
        verbose: bool,
    },
    Bench {
        number: u64,
        #[arg(short, long, default_value = "1000")]
        iterations: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Calc { number, verbose } => {
            let result = if verbose {
                print!("Final result: ");
                factorial(number, true)
            } else {
                factorial(number, false)
            };
            println!("{}! = {}", number, result);
        }
        Commands::Bench { number, iterations } => {
            println!(
                "Benchmarking factorial({}) over {} iterations...",
                number, iterations
            );

            let start = Instant::now();
            for _ in 0..iterations {
                factorial(number, false);
            }
            let duration = start.elapsed();

            println!("Average time: {:?}", duration / iterations as u32);
        }
    }

    Ok(())
}

pub fn factorial(n: u64, verbose: bool) -> u64 {
    if verbose {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
            println!("Step {}: intermediate result = {}", i, result);
        }
        result
    } else {
        (1..=n).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 6)]
    #[case(4, 24)]
    #[case(5, 120)]
    #[case(10, 3_628_800)]
    fn test_quiet(#[case] input: u64, #[case] expected: u64) {
        assert_eq!(factorial(input, false), expected);
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 6)]
    #[case(4, 24)]
    #[case(5, 120)]
    #[case(10, 3_628_800)]
    fn test_verbose(#[case] input: u64, #[case] expected: u64) {
        assert_eq!(factorial(input, true), expected);
    }
}
