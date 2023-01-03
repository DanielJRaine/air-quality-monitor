#![no_main]
#![no_std]

use air_quality_monitor as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    air_quality_monitor::exit()
}
