use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && &args[1] == "--version" {
        let version = env!("CARGO_PKG_VERSION");
        println!("App Version: {}", version);
        std::process::exit(0);
    }

    if args.len() < 2 {
        eprintln!("Usage: top <file> [lines]");
        std::process::exit(1);
    }

    let filename: &String = &args[1];
    let lines_to_read: usize = args.get(2).and_then(|l| l.parse::<usize>().ok()).unwrap_or(10);

    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if i >= lines_to_read {
            break;
        }
        println!("{}", line?);
    }

    Ok(())
}