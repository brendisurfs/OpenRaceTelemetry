//! `store.rs` contains the logic for writing sensor data to disk.
//! The target for writing this data to is a micro sd card through an Adafruit MicroSD reader.
//! We interface to the MicroSD card reader via SDMMC.
//
// TODO: Build out store writer for data logging.

use aligned::{A4, Aligned};
use block_device_driver::BlockDevice as _;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::sdmmc::Sdmmc;
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, bind_interrupts, dma, peripherals, sdmmc};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Delay;
use panic_probe as _;
use sdio::sd::Card;
use sdio::{BlockDevice, MmcBus};

// From `embassy-rs` examples, this is a safeguard for overwriting data on the sd card.
// If you don't care, you can toggle this off.
const ALLOW_WRITES: bool = true;

const BLOCK_DEVICE_FREQ: u32 = 24_000_000;

struct WriteRequest(u32, &'static [Aligned<A4, [u8; 512]>]);

pub async fn connect_sd_card_spi(sdmmc_conn: Sdmmc, peri: &Peripherals) {
    let card_config = sdmmc::Config::default();

    let storage = BlockDevice::new_sd_card(sdmmc_conn, BLOCK_DEVICE_FREQ, Delay)
        .await
        .expect("error initializing sd card");
}
