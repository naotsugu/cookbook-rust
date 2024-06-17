use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use text_colorizer::*;

fn grep<R>(target: &str, reader: R) -> io::Result<()>
        where R: BufRead {
    for (n, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if line.contains(target) {
            println!("{:>5}: {}", n, line);
        }
    }
    Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);

    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep <pattern <file-path>...")?
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            println!("\n{}", file.display().to_string().green());
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }
    Ok(())
}

fn main() {
    let result = grep_main();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
