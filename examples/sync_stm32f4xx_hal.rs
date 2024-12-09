#![no_std]
#![no_main]

use cortex_m_rt::entry;

use m24c64_driver::M24C64;
use stm32f4xx_hal::{self as hal, gpio::GpioExt, i2c::I2c, pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to the device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the system clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = cp.SYST.delay(&clocks);

    // Configure I2C1
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8;
    let sda = gpiob.pb7;
    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::standard(100.kHz()),
        &clocks,
    );

        // Create a new instance of the M24C64 EEPROM driver
    let mut m24c64_eeprom = M24C64::new_blocking(i2c, 0b000);

    loop {
        // Write bytes to the EEPROM
        let write_data = [0x10, 0x20, 0x30, 0x40];
        m24c64_eeprom
            .write_blocking(0x12, &write_data, &mut delay)
            .expect("Write failed");

        // Read the bytes back
        let mut read_data = [0u8; 4];
        m24c64_eeprom
            .read_blocking(0x12, &mut read_data)
            .expect("Read failed");

        assert_eq!(
            write_data, read_data,
            "Data read does not match data written"
        );

        delay.delay_ms(1000);
    }
}
