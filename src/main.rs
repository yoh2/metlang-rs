use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use structopt::StructOpt;

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

fn read_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    let source = if let Some(exp) = opt.expression {
        exp
    } else if let Some(file) = opt.source_file {
        if file.as_os_str() == "-" {
            read_from_stdin()?
        } else {
            fs::read_to_string(file)?
        }
    } else {
        read_from_stdin()?
    };

    let program = metlang::parse(&source)?;
    metlang::run(&program)?;

    if !opt.no_newline {
        println!();
    }
    Ok(())
}
