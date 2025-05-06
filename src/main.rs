use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};

use clap::Parser;
use nix;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the pipe to write to
    #[arg(short, long)]
    name: String,

    /// The duration of time to run the FIFO in seconds, defaults to 10
    #[arg(short = 't', long, default_value = "10")]
    duration: u64,

    /// The number of bytes to write to the pipe on each iteration
    #[arg(short = 'c', long, default_value = "1")]
    count: u64,

    /// The delay between writing bytes to the pipe in microseconds, defaults to 1000µs (1ms)
    #[arg(short = 'd', long, default_value = "1000")]
    delay: u64,

    /// The byte value to write to the pipe, defaults to 0
    #[arg(short = 'b', long, default_value = "0")]
    byte_value: u8,

    /// Sets the cpu core to run the FIFO on
    #[arg(long)]
    core: Option<usize>,
}

fn main() {
    let args = Args::parse();

    if let Some(core) = args.core {
        if let Err(e) = set_core_affinity(core) {
            eprintln!("Error setting core affinity: {}", e);
            std::process::exit(1);
        }
    }

    match open_fifo(&args.name) {
        Ok(pipe) => {
            if let Err(e) = run_fifo(pipe, &args) {
                eprintln!("Error running FIFO: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error opening FIFO: {}", e);
            std::process::exit(1);
        }
    }
}

fn set_core_affinity(core: usize) -> Result<()> {
    let core_ids = core_affinity::get_core_ids().unwrap();
    let core_id = core_ids
        .into_iter()
        .find(|id| id.id == core)
        .ok_or(anyhow!("Core not found"))?;
    core_affinity::set_for_current(core_id);
    Ok(())
}

fn open_fifo(name: &str) -> Result<File> {
    // Ensure the FIFO exists beforehand via `mkfifo /tmp/my_pipe` in shell
    if !Path::new(name).exists() {
        println!("FIFO does not exist, creating it");
        nix::unistd::mkfifo(name, nix::sys::stat::Mode::S_IRWXU)?;
    }

    println!("Opening FIFO: {}", name);

    // Open the FIFO for writing
    let pipe = OpenOptions::new().write(true).open(name)?; // Will block until the reader opens it

    Ok(pipe)
}

fn run_fifo(mut pipe: File, args: &Args) -> Result<()> {
    println!("Running FIFO for {} seconds", args.duration);

    let payload = vec![args.byte_value; args.count as usize];

    // Write 0x00 every 1ms to the pipe for the duration
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(args.duration) {
        pipe.write_all(&payload)?;
        std::thread::sleep(Duration::from_micros(args.delay));
    }

    Ok(())
}
