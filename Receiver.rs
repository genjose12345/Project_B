//Reciver file reads data from stdin(pipe)
use std::env; //Access command-line arguments
use std::error::Error; //Error handling
use std::io::{self, BufRead}; //Read from stdin line by line
use std::thread; //Thread::sleep functionality
use std::time::Duration; //Specifying sleep durations

fn main() -> Result<(), Box<dyn Error>> {
    //Read command line argument to get processing delay
    //Default to 1000ms if not specified
    let args: Vec<String> = env::args().collect();
    let delay_ms = if args.len() > 1 {
        args[1].parse::<u64>().unwrap_or(1000)
    } else {
        1000
    };

    //Print startup information to stderr
    eprintln!("Receiver starting: processing with {}ms delay", delay_ms);

    //Create a buffered reader for stdin to read data from the pipe line by line
    //stdin.lock() gets exclusive access to stdin for this thread
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    //Process each line received from the pipe until EOF
    let mut count = 0;
    while let Some(line) = reader.next() {
        //Extract the line content
        let line = line?;
        count += 1;

        //print receipt of data to stderr
        eprintln!("Receiver: received item #{}", count);

        //Process the data
        let processed = line.to_uppercase();

        //Print the processed data to stdout
        println!("PROCESSED: {}", processed);

        //Simulate processing time with a delay
        thread::sleep(Duration::from_millis(delay_ms));
    }

    //Print completion message
    eprintln!("Receiver: finished processing {} items", count);

    // Return success
    Ok(())
}
