#![no_std]
#![no_main]
#![allow(unused)]

use defmt_rtt as _;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::{self, InterruptHandler as DmaInterruptHandler};
use embassy_stm32::i2c::I2c;
use embassy_stm32::sdmmc::Sdmmc;
use embassy_stm32::time::Hertz;
use embassy_stm32::usart::{self, Uart};
use embassy_stm32::{Config, bind_interrupts, i2c, peripherals, sdmmc};
use embassy_time::Duration;
use panic_probe as _;

mod gps;
mod imu;
mod utils;

use gps::Gps;
use imu::Imu;
use utils::blink::{self, Led};

use crate::utils::store::connect_to_sd_card;

// use crate::utils::store::connect_sd_card_spi;

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

    // These are used for MicroSD card reading.
    SDIO => sdmmc::InterruptHandler<peripherals::SDIO>;
    DMA2_STREAM3 => dma::InterruptHandler<peripherals::DMA2_CH3>;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) -> ! {
    let mut config = Config::default();
    // NOTE: Im going to be really honest here:
    // I have no clue what half this setup is for sdmmc.
    // But the example shows it, and im going to figure it out.
    // All I know is that I need it. Learning as I go.
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Bypass,
        });

        config.rcc.pll_src = PllSource::HSE;

        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 168 / 2 = 168Mhz.
            divq: Some(PllQDiv::DIV7), // 8mhz / 4 * 168 / 7 = 48Mhz.
            divr: None,
        });

        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }

    let peri = embassy_stm32::init(config);

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

    // Sdmmc connect for sd card interface

    info!("Setting up SD card interface");
    let sdmmc_config = sdmmc::Config::default();

    let mut sdmmc_bus = Sdmmc::new_1bit(
        peri.SDIO,
        peri.DMA2_CH3,
        Irqs,
        peri.PB15,
        peri.PA6,
        peri.PB4,
        sdmmc_config,
    );

    connect_to_sd_card(sdmmc_bus);

    loop {
        info!("Running blink");
        blink::gps_warmup_blink(&mut led).await;
    }
}
