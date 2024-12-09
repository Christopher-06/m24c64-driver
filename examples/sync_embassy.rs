#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{i2c::I2c, time::Hertz};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use m24c64_driver::M24C64;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Configure the microcontroller
    let uc_config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(uc_config);

    // Configure I2C2
    let i2c_config = embassy_stm32::i2c::Config::default();
    let i2c = I2c::new_blocking(p.I2C2, p.PB10, p.PC12, Hertz(100_000), i2c_config);

    // Create a new instance of the M24C64 EEPROM driver
    let mut delay = Delay;
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
