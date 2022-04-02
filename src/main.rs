use std::{fs, io, path::PathBuf};

use structopt::StructOpt;

use libbf::prelude::*;

pub mod metlang;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Execute the expression
    #[structopt(short, long)]
    expression: Option<String>,

    /// Do not output the trailing newline
    #[structopt(short, long)]
    no_newline: bool,

    /// Source file
    #[structopt(name = "FILE", parse(from_os_str))]
    source_file: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    let tokenizer = metlang::tokenizer();

    let program = if let Some(exp) = opt.expression {
        parse_str(&tokenizer, &exp)?
    } else if let Some(file) = opt.source_file {
        if file.as_os_str() == "-" {
            parse(&tokenizer, &mut io::stdin())?
        } else {
            parse(&tokenizer, &mut fs::File::open(file)?)?
        }
    } else {
        parse(&tokenizer, &mut io::stdin())?
    };

    run(&program, io::stdin(), io::stdout())?;

    if !opt.no_newline {
        println!();
    }
    Ok(())
}
