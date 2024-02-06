
use std::{arch::x86_64::{self, _rdtsc}, time::{Duration, Instant}};

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// Pulse width modulation to place 100*run_us/period_us load on a core
    Pwm {
        #[arg(long)]
        period_us: u64,
        #[arg(long)]
        run_us: u64
    },
    /// Hog simply runs an infinite loop to attempt to hog an entire CPU core
    Hog,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[cfg(target_arch="x86_64")]
fn tick() -> u64 {
    unsafe {
        _rdtsc()
    }
}

/// Measure how many RDTSC ticks are in a single microsecond so we can use that
/// conversion later to convert from us to ticks when computing the loop
/// iterations required to burn a given amount of CPU time.
fn ticks_per_us() -> u64 {
    (0..8).map(|_| {
        let start = Instant::now();
        let start_ticks = tick();
        while start.elapsed().as_millis() < 100 {
            unsafe { x86_64::_mm_pause() };
        }
        let end = start.elapsed();
        let end_ticks = tick();
        (((end_ticks - start_ticks) as u128) / end.as_micros()) as u64
    }).min().unwrap()
}

fn main() {
    match Cli::parse().command {
        Commands::Pwm{period_us, run_us} => {
            let ticks_per_us = ticks_per_us();
            loop {
                let start = tick();
                let target = start + run_us * ticks_per_us;
                while tick() < target  {
                    unsafe { x86_64::_mm_pause() }
                }
                std::thread::sleep(Duration::from_micros(period_us - run_us));
            }
        },
        Commands::Hog => {
            loop {
                unsafe { x86_64::_mm_pause() }
            }
        }
    }
}