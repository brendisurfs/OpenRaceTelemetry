#![no_std]
#![no_main]

use defmt_rtt as _;

use defmt::*;
use embassy_executor::Spawner;
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

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) -> ! {
    let peri = embassy_stm32::init(Config::default());

    info!("ORT firmware started");
    let led = Led::new(peri.PC13);

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
    let mut imu = Imu::new(i2c);

    // UART setup
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

    let mut gps = Gps::new(uart);

    let mut buf = [u8; 82];
    loop {
        unwrap!(gps.read_sentence(&mut buf));
        blink::gps_warmup_blink(&mut led);
    }
}
