//This project will generate data and send it through the pipe to be consumed by another process
use std::env; //For accessing command-line arguments
use std::error::Error; //Error handling
use std::io::{self, Write}; //Writing to stdout
use std::thread; //thread::sleep functionality
use std::time::Duration; //specifying sleep durations

fn main() -> Result<(), Box<dyn Error>> {
    //Read command line arguments to get the number of items to generate
    //Default to 10 if not specified
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(10)
    } else {
        10
    };

    //Read second command line argument to get delay between items
    //Default to 500ms if not specified
    let delay_ms = if args.len() > 2 {
        args[2].parse::<u64>().unwrap_or(500)
    } else {
        500
    };

    //print startup information to stderr
    //eprintln! sends output to stderr
    eprintln!(
        "Sender starting: generating {} items with {}ms delay",
        count, delay_ms
    );

    //Main data generation loop
    for i in 1..=count {
        //Create a data item with a sequence number and current timestamp
        let data = format!(
            "Item #{}: Generated at timestamp {}",
            i,
            chrono::Local::now()
        );

        //Write the data to stdout which will then be sent through the pipe to the Reciver process
        writeln!(io::stdout(), "{}", data)?;

        //Flush stdout to ensure data is sent immediately through the pipe
        io::stdout().flush()?;

        //Print to stderr that we sent an item
        eprintln!("Sender: sent item #{}", i);

        //Pause to simulate processing time
        thread::sleep(Duration::from_millis(delay_ms));
    }

    //Print completion message
    eprintln!("Sender: finished sending all items");

    // Return success
    Ok(())
}
