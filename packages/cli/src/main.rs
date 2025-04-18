mod utils;
use utils::file::collect_files_recursively;

use std::{fs::File, io::Read, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use parser::{WooriCardParser, base::KST};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Process card transactions
    Card {
        card_issuer: CardIssuer,

        #[arg(
            short,
            long,
            default_value = "./input",
            help = "Input file or directory"
        )]
        input: PathBuf,

        #[arg(
            short,
            long,
            default_value = "transactions.rs",
            help = "Output file name"
        )]
        output: PathBuf,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CardIssuer {
    Samsung,
    BC,
    Woori,
    Hana,
    Shinhan,
    Hyundai,
    KB,
    Lotte,
    NH,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    let args = Args::parse();

    match args.command {
        Commands::Card {
            card_issuer,
            input,
            output,
        } => {
            tracing::info!("Processing card_issuer: {:?}", card_issuer);
            tracing::info!("Input path: {:?}", input);
            tracing::info!("Output path: {:?}", output);
            let files = collect_files_recursively(&input);
            match card_issuer {
                CardIssuer::Woori => {
                    let kst = KST.to_offset();
                    let mut parser = WooriCardParser::new(kst);
                    for (file_type, pathbuf) in files {
                        let mut bytes = File::open(pathbuf)?;
                        let mut buffer = Vec::new();
                        bytes.read_to_end(&mut buffer)?;

                        parser.parse(file_type, buffer)?;
                    }
                    tracing::debug!("Parsed transactions: {:?}", parser.export_transactions());
                    tracing::info!("Processing Woori card transactions");
                }
                _ => {
                    tracing::error!("Unsupported card brand: {:?}", card_issuer);
                    return Err("Unsupported card brand".into());
                }
            }
        }
    }
    Ok(())
}
