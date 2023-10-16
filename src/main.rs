use std::env;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are enough arguments
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: {} <total_length> <name> [fill_char]", args[0]);
        std::process::exit(1);
    }

    // Parse arguments
    let total_length: usize = match args[1].parse() {
        Ok(length) => length,
        Err(_) => {
            eprintln!("Error: Invalid total_length argument. It must be a positive integer.");
            std::process::exit(1);
        }
    };

    let name = &args[2];
    let fill_char = args
        .get(3)
        .map(|s| s.chars().next().unwrap())
        .unwrap_or('-');

    // Calculate the number of '-' characters needed on each side to center the name
    let name_length = name.len() + 2;
    let padding_length = (total_length - name_length) / 2;

    let repeated_string: String = std::iter::repeat(fill_char).take(padding_length).collect();
    let centered_string = format!("{0} {1} {0}", repeated_string, name);

    println!("{}", centered_string);
}
