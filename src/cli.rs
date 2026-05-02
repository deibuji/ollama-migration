use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

#[derive(Debug, Parser)]
#[command(name = "ollama-migrator")]
#[command(about = "Extract GGUF models from Ollama storage")]
pub struct Cli {
    #[arg(short, long, value_name = "PATH")]
    pub ollama_dir: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value = "table")]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List installed Ollama models
    List,

    /// Export one or more models to GGUF
    Export {
        /// Model reference(s) to export
        models: Vec<String>,

        /// Output file or directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Export all models
    ExportAll {
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Filter by pattern
        #[arg(short, long)]
        pattern: Option<String>,
    },

    /// Show model metadata
    Info {
        /// Model reference or file path
        target: String,
    },

    /// Verify a GGUF file
    Verify {
        /// GGUF file path
        path: PathBuf,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;
    use super::Cli;

    #[test]
    fn test_cli_parser() {
        Cli::command().debug_assert();
    }
}
