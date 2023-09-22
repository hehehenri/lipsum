use clap::Parser;
use rinha::interp::eval_file;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let file_buffer = std::fs::read_to_string(args.file).map_err(|err| err.to_string())?;

    let file_ast = serde_json::from_str(&file_buffer)
        .map_err(|err| err.to_string())
        .unwrap();

    let _ = eval_file(file_ast).unwrap();

    Ok(())
}
