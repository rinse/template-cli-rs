use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Branch { ref branch_name, d } =>
            if d {
                println!("Delete the branch {}", branch_name)
            } else {
                println!("Create a branch {}", branch_name)
            }
        cli::Command::Commit { ref m } =>
            match m {
                None => println!("no Commit message"),
                Some(message) => println!("Commit message: {}", message),
            }
        cli::Command::Export { format } =>
            println!("Output with the format: {:?}", format),
    }
}
