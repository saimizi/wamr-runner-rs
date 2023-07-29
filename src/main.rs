mod error;
mod wamr;

#[allow(unused)]
use {
    clap::Parser,
    error::WamrError,
    error_stack::{IntoReport, Report, Result, ResultExt},
    jlogger_tracing::{jdebug, jerror, jinfo, jwarn, JloggerBuilder, LevelFilter, LogTimeFormat},
    wamr::Wamr,
};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
struct Cli {
    /// Execute wasm byte code or AOT file with XIP supported.
    #[arg(short, long, value_name = "WASM BINARY")]
    wasm: String,

    #[arg(short, long, action=clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), WamrError> {
    let cli = Cli::parse();

    let level = match cli.verbose {
        1 => LevelFilter::DEBUG,
        2 => LevelFilter::TRACE,
        _ => LevelFilter::INFO,
    };

    JloggerBuilder::new()
        .log_console(true)
        .max_level(level)
        .log_time(LogTimeFormat::TimeStamp)
        .build();

    Wamr::run(&cli.wasm)
}
