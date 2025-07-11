use anyhow::Context;
use clap::{arg, Parser, Subcommand};
use glotaran_converter_lib::{run_das6, run_lfp, run_r4};

pub mod das {
    pub const PS_PER_NS: f32 = 1000f32;
    pub const DEFAULT_SYNC_DELAY: f32 = 200.0;
    pub const DEFAULT_NS_PER_CHN: f32 = 0.055;
    pub const DEFAULT_OUTPUT_FILENAME: &str = "converted_tres_file.ascii";
}
pub mod lfp {}
#[derive(Debug, Parser)]
#[command(
    name = "glotaran converter util",
    about = "Utility to convert lfp and das6 tr fluorescence data"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    DAS {
        /// Name of the file to convert
        filename: String,
        /// Channels before the excitatory laser pulse, if not indicated default is 200 chn.
        #[arg(short, long, value_name = "DELAY")]
        sync_delay: Option<f32>,
        /// Conversion from channels to ns, if not indicated default is 0.055 ns/chn.
        #[arg(short, long, value_name = "CONVERSION_FACTOR")]
        ns_per_chn: Option<f32>,
        #[arg(short, long)]
        output_filename: Option<String>,
    },
    LFP {
        #[arg(short, long, value_name = "filename")]
        filename: String,
        #[arg(short, long)]
        output_filename: Option<String>,
        #[arg(short = 'r', long, default_value_t = false)]
        r4: bool,
    },
}

fn das6flow(
    filename: &str,
    sync_delay: Option<f32>,
    ns_per_chn: Option<f32>,
    output_filename: Option<String>,
) -> anyhow::Result<()> {
    let sync_delay = sync_delay.unwrap_or(das::DEFAULT_SYNC_DELAY);
    let ns_per_chn = ns_per_chn.unwrap_or(das::DEFAULT_NS_PER_CHN);
    let output_filename = match output_filename.unwrap_or(das::DEFAULT_OUTPUT_FILENAME.into()) {
        x if x.ends_with("ascii") => x,
        x => format!("{x}.ascii"),
    };
    println!("Corriendo conversion de archivo, {filename}");
    println!("Parámetros");
    println!("Sync delay \t\t\t {sync_delay} chn");
    println!("Equivalencia \t\t\t {ns_per_chn} ns/chn");
    run_das6(
        filename,
        sync_delay,
        ns_per_chn * das::PS_PER_NS,
        output_filename,
    )
    .context("Error convirtiendo archivo de fluorescencia")?;

    return anyhow::Ok(());
}

fn lfpflow(filename: &str, _output_filename: Option<String>, r4: bool) -> anyhow::Result<()> {
    if r4 {
        run_r4(filename.to_owned()).context("Problema con r4 file")?;
    } else {
        run_lfp(filename).context("Problem running file")?;
    }
    return anyhow::Ok(());
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::LFP {
            filename,
            output_filename,
            r4,
        } => lfpflow(&filename, output_filename, r4),
        Commands::DAS {
            filename,
            sync_delay,
            ns_per_chn,
            output_filename,
        } => das6flow(&filename, sync_delay, ns_per_chn, output_filename),
    }
    .context("Error convirtiendo datos:")?;
    return anyhow::Ok(());
}
