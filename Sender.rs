// Producer: Generates data and sends it through stdout to be consumed by another process
use std::env; // For accessing command-line arguments
use std::error::Error; // For error handling with Result and Box<dyn Error>
use std::io::{self, Write}; // For writing to stdout and flushing the buffer
use std::thread; // For thread::sleep functionality
use std::time::Duration; // For specifying sleep durations

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments to get the number of items to generate
    // Default to 10 if not specified or parsing fails
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(10)
    } else {
        10
    };

    // Parse second command line argument to get delay between items
    // Default to 500ms if not specified or parsing fails
    let delay_ms = if args.len() > 2 {
        args[2].parse::<u64>().unwrap_or(500)
    } else {
        500
    };

    // Log startup information to stderr (not part of the pipe)
    // eprintln! sends output to stderr, which won't be sent through the pipe
    eprintln!(
        "Producer starting: generating {} items with {}ms delay",
        count, delay_ms
    );

    // Main data generation loop - create and send 'count' number of items
    for i in 1..=count {
        // Create a data item with a sequence number and current timestamp
        let data = format!(
            "Item #{}: Generated at timestamp {}",
            i,
            chrono::Local::now()
        );

        // Write the data to stdout, which will be piped to the consumer process
        // The ? operator propagates any errors that might occur during writing
        writeln!(io::stdout(), "{}", data)?;

        // Flush stdout to ensure data is sent immediately through the pipe
        // Without this, data might be buffered and not sent until buffer is full
        io::stdout().flush()?;

        // Log to stderr that we sent an item (this output won't go through the pipe)
        eprintln!("Producer: sent item #{}", i);

        // Pause to simulate processing time / production rate
        thread::sleep(Duration::from_millis(delay_ms));
    }

    // Log completion message
    eprintln!("Producer: finished sending all items");

    // Return success
    Ok(())
}
