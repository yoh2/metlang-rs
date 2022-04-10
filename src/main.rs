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

    let parser = metlang::parser();

    let program = if let Some(exp) = opt.expression {
        parser.parse_str(&exp)?
    } else if let Some(file) = opt.source_file {
        if file.as_os_str() == "-" {
            parser.parse(&mut io::stdin())?
        } else {
            parser.parse(&mut fs::File::open(file)?)?
        }
    } else {
        parser.parse(&mut io::stdin())?
    };

    if let Err(e) = runtime::run(&program, io::stdin(), io::stdout()) {
        if let RuntimeError::Eof = e {
            // EOF is a normal case.
        } else {
            return Err(e.into());
        }
    }

    if !opt.no_newline {
        println!();
    }
    Ok(())
}
