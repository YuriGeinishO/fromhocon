use std::io::Read;
use anyhow::{
    anyhow,
    Context,
};
use crate::converter::Converter;

mod converter;

fn main() -> anyhow::Result<()> {
    
    let mut loader = hocon::HoconLoader::new();

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("# Usage");
        println!();
        println!("    fromhocon <spec...>");
        println!("    fromhocon --version");
        println!();
        println!("Where `spec` is one of");
        println!("  - `--stdin` means read HOCON string from stdin");
        println!("  - `--file <path>` means read HOCON from the `path`");
        println!("  - `--adhoc <hocon-string>` means take HOCON from `hocon-string`");
        println!();
        println!("Later specs override values of the earlier specs when keys overlap.");
        return Ok(());
    }

    if args[0] == "--version" {
        println!(clap::crate_version!());
        return Ok(());
    }

    let mut args_iter = args.iter().peekable();
    while args_iter.peek().is_some() {
        let config_source = args_iter.next().unwrap();
        match config_source.as_str() {
            "--stdin" => {
                let mut buffer = String::new();
                std::io::stdin()
                    .read_to_string(&mut buffer)
                    .map_err(|e| hocon::Error::Io { message: e.to_string() })?;
                loader = loader.load_str(&buffer).context("Loading HOCON from stdin")?;
            },
            "--file" => {
                let path = args_iter.next().context("--file is missing a following path")?;
                loader = loader.load_file(path).context(format!("Loading HOCON from file {path}"))?;
            },
            "--adhoc" => {
                let string = args_iter.next().context("--adhoc is missing a following HOCON string")?;
                loader = loader.load_str(&string).context("Loading HOCON from adhoc string `{string}`")?;
            },
            _ => return Err(anyhow!("Unknown config source `{config_source}`")),
        }
    }

    let parsed = loader.hocon().context("Building the final HOCON object")?;

    Converter::run(parsed).map(|output| println!("{}", output))?;

    Ok(())
}
