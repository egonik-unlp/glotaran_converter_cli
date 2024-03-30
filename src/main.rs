use std::fmt::format;

use clap::{Parser, arg}; 
use glotaran_converter_lib::run;
#[derive(Parser)]
struct Cli {
    /// Name of the file to convert
    filename : String,
    /// Channels before the excitatory laser pulse, if not indicated default is 200 chn.
    #[arg(short, long, value_name = "DELAY")]
    sync_delay : Option<f32>,
    /// Conversion from channels to ns, if not indicated default is 0.055 ns/chn.
    #[arg(short, long, value_name = "CONVERSION_FACTOR")]
    ns_per_chn : Option<f32>,
    #[arg(short, long)]
    output_filename : Option<String>

}

const PS_PER_NS : f32 = 1000f32;
const DEFAULT_SYNC_DELAY : f32 = 200.0;
const DEFAULT_NS_PER_CHN : f32 = 0.055;
const DEFAULT_OUTPUT_FILENAME : &str = "converted_tres_file.ascii";

fn main() {
    let args = Cli::parse();
    let sync_delay = args.sync_delay.unwrap_or(DEFAULT_SYNC_DELAY);
    let ns_per_chn = args.ns_per_chn.unwrap_or(DEFAULT_NS_PER_CHN);
    let output_filename = match args.output_filename.unwrap_or(DEFAULT_OUTPUT_FILENAME.into()) {
        x if x.ends_with("ascii") => x,
        x => format!("{}.ascii", x)
    };
    println!("Corriendo conversion de archivo, {}", args.filename);
    println!("Parámetros");
    println!("Sync delay \t\t\t {} chn", sync_delay);
    println!("Equivalencia \t\t\t {} ns/chn", ns_per_chn);
    match run_das6(&args.filename, sync_delay, ns_per_chn * PS_PER_NS, output_filename) {
        Ok(filename) => println!("Se convirtió exitosamente el archivo {}", filename),
        Err(e) => println!("No se pudo convertir el archivo exitosamente porque  {}", e)
    };
}
