use clap::{Parser, Subcommand};
use std::path::PathBuf;
use textfsm_rs::{CliTable, DataRecordConversion, TextFSM};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a file using a specific TextFSM template
    Parse {
        /// Path to the TextFSM template file
        #[arg(short, long)]
        template: PathBuf,

        /// Path to the input data file
        #[arg(short, long)]
        input: PathBuf,

        /// Convert keys to lowercase
        #[arg(short, long)]
        lowercase: bool,
    },
    /// Use CLI Table (ntc-templates index) to parse data
    Auto {
        /// Path to the index file (e.g. ntc_templates/templates/index)
        #[arg(long)]
        index: PathBuf,

        /// Platform name (e.g. cisco_ios)
        #[arg(short, long)]
        platform: String,

        /// Command executed (e.g. "show version")
        #[arg(short, long)]
        command: String,

        /// Path to the input data file
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse {
            template,
            input,
            lowercase,
        } => {
            let template_str = template.to_str().expect("Invalid template path");
            let input_str = input.to_str().expect("Invalid input path");

            let mut fsm = TextFSM::from_file(template_str)?;
            let conversion = if *lowercase {
                Some(DataRecordConversion::LowercaseKeys)
            } else {
                None
            };

            let results = fsm.parse_file(input_str, conversion)?;
            let yaml_output = serde_yml::to_string(&results)?;
            println!("{}", yaml_output);
        }
        Commands::Auto {
            index,
            platform,
            command,
            input,
        } => {
            let index_str = index.to_str().expect("Invalid index path");
            let input_str = input.to_str().expect("Invalid input path");

            let cli_table = CliTable::from_file(index_str)?;

            if let Some((template_dir, row)) = cli_table.get_template_for_command(platform, command)
            {
                // ntc-templates index often points to relative paths.
                // We need to resolve the template path relative to the index file or the templates dir.
                // The library returns the directory of the index file as template_dir if using `from_file`.

                // Typically we try the first template in the list
                if let Some(template_name) = row.templates.first() {
                    let template_path = PathBuf::from(&template_dir).join(template_name);
                    eprintln!("Using template: {}", template_path.display());

                    let mut fsm = TextFSM::from_file(template_path.to_str().unwrap())?;
                    let results =
                        fsm.parse_file(input_str, Some(DataRecordConversion::LowercaseKeys))?;

                    let yaml_output = serde_yml::to_string(&results)?;
                    println!("{}", yaml_output);
                } else {
                    anyhow::bail!("No template found in index row");
                }
            } else {
                anyhow::bail!(
                    "No matching template found for platform '{}' and command '{}'",
                    platform,
                    command
                );
            }
        }
    }

    Ok(())
}
