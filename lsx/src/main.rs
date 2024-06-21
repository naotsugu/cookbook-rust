use std::path::Path;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);

    let path = args.next().unwrap_or_else(|| "./".to_string());
    let path = Path::new(&path);

    println!("\x1b[93m{}\x1b[0m", fs::canonicalize(&path)?.display());
    if path.is_dir() {
        for entry_result in path.read_dir()? {
            let entry = entry_result?;
            println!("  {}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}
