use clap::{Parser, ValueEnum};

const DEFAULT_TICK_MS: u64 = 33; // ~30 fps

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short = 't',
        long,
        default_value_t = DEFAULT_TICK_MS,
        value_parser = clap::value_parser!(u64).range(1..=1000)
    )]
    pub tick_ms: u64,

    #[arg(short = 'p', long, value_enum, default_value = "centis")]
    pub precision: Precision,
}
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precision {
    Seconds,
    Centis,
    Millis,
}
