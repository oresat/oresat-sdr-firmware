#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;

// -> !, lets probe-rs print a backtrace on panic
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let brd = embassy_stm32::init(Default::default());
    // Probably will need to change PC13 to a different pin?
    let mut led = Output::new(brd.PC13, Level::Low, Speed::High);

    loop {
        led.set_high();
        let _ = Timer::after_millis(500).await;

        led.set_low();
        let _ = Timer::after_millis(500).await;
        info!("led flasheddddddd");
    }
}
