use std::fs;

use clap::{Parser, Subcommand};
use log::info;
use semgrep_rs;

// clap CLI struct.
#[derive(Parser, Debug)]
#[command(
    override_usage = "./semgrep-rs-example ZZZZ -r path/to/rules/ [-p path/to/policies/] [-s 9090] [-q]"
)]
#[command(version = "0.1")]
#[command(about = "semgrep-rs usage example", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

// subcommands
#[derive(Subcommand, Debug)]
enum Action {
    /// combines rules into one file
    Combine {
        /// paths to the rules directories or files
        paths: Vec<String>,

        /// path to the output file
        #[arg(short, long = "output")]
        output: String,
    },
    // /// runs Semgrep
    // Run {
    //     /// path to the rule(s) file or directory
    //     #[arg(short, long = "path")]
    //     path: String,
    // },
}

fn main() {
    // parse stuff
    let cli = Cli::parse();
    match cli.action {
        Action::Combine {
            paths: rules,
            output,
        } => run_combine(&rules, &output),
    };
}

// Combine rules in paths and write it to output.
fn run_combine(paths: &Vec<String>, output: &str) {
    // Convert Vec<String> to Vec<&str>.
    let r: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
    // Create a rule index from all paths. Inaccessible and hidden files/paths
    // will be ignored. Panic on fatal errors.
    let rule_index = semgrep_rs::GenericRuleIndex::from_paths_simple(r).unwrap();
    // Create a YAML file from all rules and panic on errors.
    let content = rule_index.get_all().to_string().unwrap();
    // Write the yaml file to disk.
    fs::write(output, content).expect("couldn't write the rule file");
    info!("Wrote the combined rule file to: {}", output);
}
