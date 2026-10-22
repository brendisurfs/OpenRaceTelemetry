//! `store.rs` contains the logic for writing sensor data to disk.
//! The target for writing this data to is a micro sd card through an Adafruit MicroSD reader.
//! We interface to the MicroSD card reader via SDMMC.
//
// TODO: Build out store writer for data logging.

use embassy_stm32::{
    Peri, Peripherals, bind_interrupts, sdmmc, spi::{MisoPin, Spi},
};

// From `embassy-rs` examples, this is a safeguard for overwriting data on the sd card.
// If you don't care, you can toggle this off.
const ALLOW_WRITES: bool = true;


struct WriteRequest(u32, &'static [Aligned<A4, [u8; 512]>]);

fn connect_sd_card_spi(peri: &Peripherals) {
    let card_config = sdmmc::Config::default();

    let p = 
}
