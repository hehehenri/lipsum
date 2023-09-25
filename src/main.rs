use clap::Parser;
use rinha::interp::eval_file;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

static DEFAULT_PATH: &'static str = "/var/rinha/source.rinha.json";

fn main() -> Result<(), String> {
    let args = Args::parse();
    let path = match args.file {
        Some(file) => file,
        None => DEFAULT_PATH.to_string(),
    };

    let file_buffer = std::fs::read_to_string(path).expect("failed to load source");

    let file_ast = serde_json::from_str(&file_buffer).expect("failed to parse ast");

    let _ = eval_file(file_ast).unwrap();

    Ok(())
}
