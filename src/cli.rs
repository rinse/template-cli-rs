use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, PartialEq, Eq, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub enum Command {
    Echo {
        message: Vec<String>,
        #[arg(short)]
        n: bool,
    },
    Export {
        format: ExportFormat,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ExportFormat {
    Json, Yaml,
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use super::*;

    #[test]
    fn echo() {
        let cli = Cli::parse_from(vec!["prog", "echo", "Hello", "World"]);
        assert_eq!(cli, Cli {
            command: Command::Echo {
                message: ["Hello", "World"].map(|e| e.to_string()).into_iter().collect(),
                n: false,
            }
        })
    }

    #[test]
    fn echo_n() {
        let cli = Cli::parse_from(vec!["prog", "echo", "-n", "Hello", "World"]);
        assert_eq!(cli, Cli {
            command: Command::Echo {
                message: ["Hello", "World"].map(|e| e.to_string()).into_iter().collect(),
                n: true,
            },
        })
    }

    #[test]
    fn export() {
        let cli = Cli::parse_from(vec!["prog", "export", "yaml"]);
        assert_eq!(cli, Cli {
            command: Command::Export {
                format: ExportFormat::Yaml,
            },
        })
    }
}
