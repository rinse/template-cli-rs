use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, PartialEq, Eq, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub enum Command {
    Branch {
        branch_name: String,    // Branch name is required
        #[clap(short)]
        d: bool,                // Delete branch
    },
    Commit {
        #[clap(short)]
        m: Option<String>,      // Optionally takes a message
    },
    Export {
        #[arg(short, long, value_enum)]
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
    fn commit() {
        let cli = Cli::parse_from(vec!["prog", "commit"]);
        assert_eq!(cli, Cli {
            command: Command::Commit {
                m: None,
            }
        })
    }

    #[test]
    fn commit_message() {
        let cli = Cli::parse_from(vec!["prog", "commit", "-m", "Hello, World"]);
        assert_eq!(cli, Cli {
            command: Command::Commit {
                m: Some("Hello, World".to_string()),
            }
        })
    }

    #[test]
    fn branch() {
        let cli = Cli::parse_from(vec!["prog", "branch", "dev"]);
        assert_eq!(cli, Cli {
            command: Command::Branch {
                branch_name: "dev".to_string(),
                d: false,
            },
        })
    }

    #[test]
    fn delete_branch() {
        let cli = Cli::parse_from(vec!["prog", "branch", "-d", "dev"]);
        assert_eq!(cli, Cli {
            command: Command::Branch {
                branch_name: "dev".to_string(),
                d: true,
            },
        })
    }

    #[test]
    fn export() {
        let cli = Cli::parse_from(vec!["prog", "export", "--format", "yaml"]);
        assert_eq!(cli, Cli {
            command: Command::Export {
                format: ExportFormat::Yaml,
            },
        })
    }
}
