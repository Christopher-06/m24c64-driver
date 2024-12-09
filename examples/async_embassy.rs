#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{i2c::I2c, time::Hertz};
use embassy_time::{Delay, Timer};
use m24c64_driver::M24C64;
use embassy_stm32::{bind_interrupts, i2c, peripherals};

bind_interrupts!(struct Irqs {
    I2C2_EV => i2c::EventInterruptHandler<peripherals::I2C2>;
    I2C2_ER => i2c::ErrorInterruptHandler<peripherals::I2C2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Configure the microcontroller
    let uc_config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(uc_config);

    // Configure I2C2
    let i2c_config = embassy_stm32::i2c::Config::default();
    let i2c = I2c::new(
        p.I2C2,
        p.PB10,
        p.PC12,
        Irqs,
        p.DMA1_CH7,
        p.DMA1_CH3,
        Hertz(100_000),
        i2c_config,
    );

    // Create a new instance of the M24C64 EEPROM driver
    let mut delay = Delay;
    let mut m24c64_eeprom = M24C64::new(i2c, 0b000);

    loop {
        // Write bytes to the EEPROM
        let write_data = [0x10, 0x20, 0x30, 0x40];
        m24c64_eeprom
            .write(0x12, &write_data, &mut delay)
            .await
            .expect("Write failed");

        // Read the bytes back
        let mut read_data = [0u8; 4];
        m24c64_eeprom
            .read(0x12, &mut read_data)
            .await
            .expect("Read failed");

        assert_eq!(
            write_data, read_data,
            "Data read does not match data written"
        );

        Timer::after_secs(1).await;
    }
}
