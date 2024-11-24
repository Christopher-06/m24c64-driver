// Based off the m24c64 crate, with some changes to support writing arbitrary lengths of data
#![cfg_attr(not(test), no_std)]

#![doc = include_str!("../README.md")]

pub mod sync;
pub mod async_variant;

/// M24C64 EEPROM Driver
pub struct M24C64<I2C> {
  /// I2C Interface
  i2c: I2C,
  /// Address set by the E pins
  e_addr: u8,
  /// Command Buffer
  cmd_buf: [u8; 34]
}

impl<I2C> M24C64<I2C> {
  /// Create a new instance of the M24C64 Driver
  /// # Arguments
  /// * `i2c` - I2C Interface (from the embedded-hal(-async) crate)
  /// * `e_addr` - The address set on the E pins
  /// 
  /// # Example
  /// ```
  /// use m24c64_driver::M24C64;
  /// 
  /// let eeprom = M24C64::new(i2c, 0);
  /// ```
  pub fn new(i2c: I2C, e_addr: u8) -> Self {
    Self {
      i2c, e_addr,
      cmd_buf: [0u8; 34]
    }
  }
}