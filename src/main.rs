use clap::Parser;
use lipsum::{
    ast::File,
    interpreter::{eval, Cache, Context, IO},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Command {
    #[arg(short, long)]
    file: Option<String>,
}

static DEFAULT_PATH: &'static str = "/var/rinha/source.rinha.json";

fn main() -> Result<(), String> {
    let command = Command::parse();
    let path = match command.file {
        Some(path) => path,
        None => DEFAULT_PATH.to_string(),
    };

    let file = std::fs::read_to_string(&path).expect(&format!("failed to read file at {}", &path));

    let parsed_file: File = serde_json::from_str(&file).unwrap();

    let entrypoint = Box::new(parsed_file.expression);

    let mut context = Context::new();
    let mut cache = Cache::new();
    let mut io = IO {};
    let _ = eval(entrypoint, &mut context, &mut cache, &mut io).unwrap();

    Ok(())
}
