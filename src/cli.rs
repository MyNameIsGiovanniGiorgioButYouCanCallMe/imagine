use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Imagine",
  about = "A simple command-line interface in Rust to categorize images",
  //todo:
  usage = "",
  //todo:
  long_about = "you've been helped."
)]
struct Cli {
    //
    #[structopt(short = "r", long = "read", help = "Path to the directory containing images")]
    // Option<String> means the input is optional - either Some(String) or None.
    read: Option<String>,

    #[structopt(short = "m", long = "mode",
    help = "Specifies the mode: \nI) for '--read': 'fast', 'slow'\nII) for '--write': 'move', 'copy' "
    )]
    mode: Option<String>,

    #[structopt(short = "w", long = "write", help = "Re-write the files in a more organized fashion.")]
    write: Option<String>,

    #[structopt(short = "c", long = "confirm", help = "Confirm the input data and display the setup")]
    confirm: bool,

    // Add more command-line options as needed
}

pub fn run_cli() {
  // Greeting
    println!("Welcome!");
    let args = Cli::from_args();

    if args.read.is_none() && args.write.is_none() {
      println!("No input provided");

    // Use read to create the ordered and hashed file
    } else if args.read.is_some() {
      match &args.mode {
        Some(_mode) => println!("read with mode {}", _mode),
        None => println!("default read"),
      }
    // Write/move the ordered images to a specific file.
    } else if args.write.is_some() {
        match &args.mode {
          Some(_mode) => println!("write with mode {}", _mode),
          None => println!("default write"),
        }

    } else {
      println!("Command has not enough arguments. Use --help for usage information.");
      return;
    }

    //todo: add confirmation
    // Display the input data if confirm is true
    if args.confirm {
      println!("Confirmed input data:");
      if let Some(read) = &args.read {
          println!("Reading from: {}", read);
      }

      if let Some(mode) = &args.mode {
          println!("Mode: {}", mode);
      }

      if let Some(write) = &args.write {
          println!("Writing to: {}", write);
      }

    } else {
        println!("Input data not confirmed. Use --confirm to display the setup.");
        return;
    }
    //todo: excecute specified actions.
    // Execute logic and calculations.

    // Completed message
    if let Some(read) = &args.read {
      println!("Categorizing images in path: {}", read);
      return;
  }
}