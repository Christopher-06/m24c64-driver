# m24c64-driver
[![crates.io](https://img.shields.io/crates/v/m24c64-driver.svg)](https://crates.io/crates/m24c64-driver)
[![Documentation](https://docs.rs/m24c64-driver/badge.svg)](https://docs.rs/m24c64-driver)

A Rust embedded-hal(-async) driver for the M24C64 I2C EEPROM, featuring arbitrary-length read/writes, timeout behaviour and asynchronous actions.

## Add to your project
```
cargo add m24c64-driver
```

## Feature
- **sync** include Synchronous Driver API (embedded-hal I2C Trait impl)
- **async** include Asynchronous Driver API (embedded-hal-async I2C Trait impl)
- **defmt** add error logging and action tracing for this crate

## Examples
Synchronous API with STM32F4XX-hal crate. All examples only work with correctly configured controllers and projects. More details can be found in the corresponding HAL documentation
```rust
use m24c64_driver::M24C64;

let mut delay = cp.SYST.delay(&clocks); // stm32f4xx-hal
// let mut delay = embassy_time::Delay; // embassy-time

let eeprom = M24C64::new_blocking(i2c, 0b000);
eeprom.write_blocking(0xA0, &[0x00, 0x01, 0x02, 0x03], &delay);

let mut my_buf = [0u8; 4];
eeprom.read_blocking(0xA0, &mut my_buf);
// my_buf = [0x00, 0x01, 0x02, 0x03]
```

Note the use of [`embedded_hal::blocking::delay::DelayMs`], which is used to retry the write every 1ms until it either succeeds, or 10ms has passed (2*t_w in the M24C64 datasheet).