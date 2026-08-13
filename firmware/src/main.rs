#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_stm32::{
    Config,
    gpio::{Level, Output, Speed},
};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    let config = Config::default();
    let peri = embassy_stm32::init(config);

    let mut led = Output::new(peri.PC13, Level::High, Speed::Low);

    loop {
        info!("hi");
        led.set_high();

        Timer::after_millis(100).await;

        led.set_low();
        Timer::after_secs(1).await;
    }
}
