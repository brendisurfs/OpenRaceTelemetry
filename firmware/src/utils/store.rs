//! `store.rs` contains the logic for writing sensor data to disk.
//! The target for writing this data to is a micro sd card through an Adafruit MicroSD reader.
//! We interface to the MicroSD card reader via SDMMC.
//
// TODO: Build out store writer for data logging.

use block_device_driver::BlockDevice as _;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::sdmmc::sd::{Addressable, CmdBlock, DataBlock, StorageDevice};
use embassy_stm32::sdmmc::{self, Sdmmc};
use embassy_stm32::time::Hertz;

use embassy_stm32::{Config, Peripherals, bind_interrupts, dma, peripherals};
use embassy_time::Delay;
use panic_probe as _;

// From `embassy-rs` examples, this is a safeguard for overwriting data on the sd card.
// If you don't care, you can toggle this off.
const ALLOW_WRITES: bool = true;

const BLOCK_DEVICE_FREQ: u32 = 24_000_000;

/// Connects to the SD card reader.
/// In its current implementation, it just reads out the storages byte size.
///
/// Returns
pub async fn connect_to_sd_card(mut sdmmc_conn: Sdmmc<'static>) {
    let card_config = sdmmc::Config::default();
    let mut cmd_block = CmdBlock::new();
    let freq = Hertz(4_000_000);
    let Ok(mut storage) = StorageDevice::new_sd_card(&mut sdmmc_conn, &mut cmd_block, freq).await
    else {
        return;
    };

    info!("Card size: {} bytes", storage.card().size());
    let mut block = DataBlock::new();

    if let Err(why) = storage.read_block(0, &mut block).await {
        error!("Read failed");
        return;
    }

    info!("MBR sig: {=[u8]:02x}", block[510..512]);
}
