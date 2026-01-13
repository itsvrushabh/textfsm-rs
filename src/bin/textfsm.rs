use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use textfsm_rs::{CliTable, DataRecordConversion, TextFSM};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Yaml, global = true)]
    format: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
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

                let mut fsm = TextFSM::from_file(template)?;

                let conversion = if *lowercase {

                    Some(DataRecordConversion::LowercaseKeys)

                } else {

                    None

                };

    

                let results = fsm.parse_file(input, conversion)?;

                

                match cli.format {

                    OutputFormat::Json => {

                        println!("{}", serde_json::to_string_pretty(&results)?);

                    }

                    OutputFormat::Yaml => {

                        println!("{}", serde_yml::to_string(&results)?);

                    }

                }

            }

            Commands::Auto {

                index,

                platform,

                command,

                input,

            } => {

                let cli_table = CliTable::from_file(index)?;

                

                if let Some((template_dir, row)) = cli_table.get_template_for_command(platform, command) {

                    if let Some(template_name) = row.templates.first() {

                        let template_path = PathBuf::from(&template_dir).join(template_name);

                        eprintln!("Using template: {}", template_path.display());

                        

                        let mut fsm = TextFSM::from_file(&template_path)?;

                        let results = fsm.parse_file(input, Some(DataRecordConversion::LowercaseKeys))?;

                        

                        match cli.format {

                            OutputFormat::Json => {

                                println!("{}", serde_json::to_string_pretty(&results)?);

                            }

                            OutputFormat::Yaml => {

                                println!("{}", serde_yml::to_string(&results)?);

                            }

                        }

                    } else {

                        anyhow::bail!("No template found in index row");

                    }

                } else {

                    anyhow::bail!("No matching template found for platform '{}' and command '{}'", platform, command);

                }

            }

        }

    Ok(())
}
