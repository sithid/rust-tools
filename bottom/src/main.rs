use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    println!(""); // Start off with a little space from the preceding prompt for clarity.

    // A vector is Rust's implementation of a resizable array, stored on the heap.
    let args: Vec<String> = env::args().collect(); 

    // Check for --version command line argument and display the version accordingly.
    if args.len() == 2 && &args[1] == "--version" {
        let version = env!("CARGO_PKG_VERSION");
        println!("App Version: {}", version);
        return Ok(());
    }

    // At this point, we know the command argument was not --version.
    // Verify that there are enough command line arguments supplied.
    // Display usage information and terminate immediately if no arguments are supplied.
    if args.len() < 2 {
        eprintln!("Usage: bottom <file> [starting_line] Optional: --numbered");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not enough arguments"));
    }

    let numbered: bool = args.len() == 4 && &args[3] == "--numbered";

    // Create a reference to the value of args[1] (the filename). We do not require ownership of the address in memory.
    let filename = &args[1];

    // Parse the optional starting line (args[2]) to read, defaulting to 10.
    let mut starting_line = args.get(2)
                            .and_then(|l| l.parse::<usize>().ok()) // Try to parse the string to usize, return None on error.
                            .unwrap_or(10);                        // Use 10 if the argument is missing or parsing fails.
    
    // Offset by 1 to make starting_line inclusive.
    if starting_line > 0 {
        starting_line = starting_line - 1;
    }

    // Open the file in read-only mode.
    let file = File::open(filename)?;

    // Initialize a reader using the opened file.
    let reader = BufReader::new(file);

    println!("{} {} {}.\n\r", &args[0], &args[1], starting_line);

    // Enumerate through each line of file and print the value read until we reach are lines_to_read limit.
    for (i, line) in reader.lines().enumerate() {
        if i < starting_line {
             continue;
        }

        if numbered {
            println!("{}:{}", i + 1, line?);   
        } else {
            println!("{}", line?);
        }
    }
    
    println!(""); // End with a little space from the following prompt for clarity.
    // return Result OK
    return Ok(());
}