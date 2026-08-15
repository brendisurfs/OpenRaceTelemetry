#![no_std]
#![no_main]

use defmt_rtt as _;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::InterruptHandler as DmaInterruptHandler;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::usart::{self, Uart};
use embassy_stm32::{Config, bind_interrupts, i2c, peripherals};
use embassy_time::Duration;
use panic_probe as _;

mod blink;
mod gps;
mod imu;

use blink::Led;
use gps::Gps;
use imu::Imu;

const BLINK_INTERVAL: Duration = Duration::from_millis(400);
const I2C_FREQUENCY: Hertz = Hertz(400_000);

// reference: https://www.st.com/resource/en/reference_manual/rm0383-stm32f411xce-advanced-armbased-32bit-mcus-stmicroelectronics.pdf
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    // DMA_CHn type is pinned to stream n, so the stream number in the interrupt name should match the channel number in the type.
    DMA1_STREAM0 => DmaInterruptHandler<peripherals::DMA1_CH0>; // I2C1_RX
    DMA1_STREAM6 => DmaInterruptHandler<peripherals::DMA1_CH6>; // I2C1_TX
    // This is for the USART stream
    DMA2_STREAM7 => DmaInterruptHandler<peripherals::DMA2_CH7>; // USART1_TX
    DMA2_STREAM2 => DmaInterruptHandler<peripherals::DMA2_CH2>; // USART1_RX
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) -> ! {
    let peri = embassy_stm32::init(Config::default());

    info!("ORT firmware started");
    let mut led = Led::new(peri.PC13.into());

    // IMU setup
    let i2c_config = i2c::Config::default();
    let i2c = I2c::new(
        peri.I2C1,
        peri.PB6,
        peri.PB7,
        peri.DMA1_CH6,
        peri.DMA1_CH0,
        Irqs,
        i2c_config,
    );

    info!("setting up imu");
    let mut imu = Imu::new(i2c);
    // imu.setup().await.expect("failed to set up IMU");

    // UART setup
    info!("configuring uart");
    let mut uart_config = usart::Config::default();
    uart_config.baudrate = gps::GPS_BAUD;

    let uart = Uart::new(
        peri.USART1,
        peri.PA10,
        peri.PA9,
        peri.DMA2_CH7,
        peri.DMA2_CH2,
        Irqs,
        uart_config,
    )
    .expect("Uart build");

    info!("Setting up gps");
    // let mut gps = Gps::new(uart);

    loop {
        info!("Running blink");
        blink::gps_warmup_blink(&mut led).await;
    }
}
