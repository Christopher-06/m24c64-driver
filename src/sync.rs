use crate::M24C64;
use embedded_hal::{delay::DelayNs, i2c};

#[cfg(feature = "defmt")]
use defmt;

impl<I2C, E> M24C64<I2C>
where
    I2C: i2c::I2c<Error = E>,
    E: i2c::Error,
{
    fn write_raw_blocking(
        &mut self,
        address: usize,
        bytes: &[u8],
        delay: &mut dyn DelayNs,
    ) -> Result<(), E> {
        self.cmd_buf[0] = (address >> 8) as u8;
        self.cmd_buf[1] = (address & 0xFF) as u8;
        self.cmd_buf[2..(bytes.len() + 2)].copy_from_slice(bytes);

        // Wait until the device is connected to the bus
        // After a write, the EEPROM disconnects itself from the bus until it can perform the write internally,
        // thus we have to continually poll the i2c bus for the device to be ready to receive new bytes.
        // while self.i2c.write(self.e_addr | 0x50, &[]).is_err() { }

        // Slight modification - keep track of the retries, since if we're over t_w from the datasheet, we want
        // to report an error instead of infinitely looping.
        let mut i = 0;
        loop {
            let result = self
                .i2c
                .write(self.e_addr | 0x50, &self.cmd_buf[0..bytes.len() + 2]);

            match result {
                Ok(_) => {
                    #[cfg(feature = "defmt")]
                    defmt::trace!(
                        "[EEPROM M24C64 Unit {:b}] Writing to address {:#x} took {} tries",
                        self.e_addr,
                        address,
                        i + 1
                    );

                    return Ok(());
                }
                Err(_) if i < 10 => (),
                Err(e) => {
                    #[cfg(feature = "defmt")]
                    defmt::error!(
                        "[EEPROM M24C64 Unit {:b}] Writing to address {:#x} failed: {}",
                        self.e_addr,
                        address,
                        e.kind()
                    );

                    return Err(e);
                }
            }

            i += 1;
            delay.delay_ms(1)
        }
    }

    fn read_raw_blocking(&mut self, address: usize, bytes: &mut [u8]) -> Result<(), E> {
        self.cmd_buf[0] = (address >> 8) as u8;
        self.cmd_buf[1] = (address & 0xFF) as u8;

        self.i2c
            .write_read(self.e_addr | 0x50, &self.cmd_buf[0..2], bytes)
    }

    /// Create a new blocking / synchronous instance of the M24C64 EEPROM driver
    /// # Arguments
    /// * `i2c` - I2C Interface (from the embedded-hal crate)
    /// * `e_addr` - The address set on the E pins
    pub fn new_blocking(i2c: I2C, e_addr: u8) -> Self {
        Self::new(i2c, e_addr)
    }

    /// Write an arbitrary number of bytes into the EEPROM, starting at `address`.
    /// This function will automatically paginate.
    pub fn write_blocking(
        &mut self,
        address: usize,
        data: &[u8],
        delay: &mut dyn DelayNs,
    ) -> Result<(), E> {
        #[cfg(feature = "defmt")]
        defmt::info!(
            "[EEPROM M24C64 Unit {:b}] Writing to address {:#x} with {} bytes",
            self.e_addr,
            address,
            data.len()
        );

        // Chunk the write into pages
        let mut i = address;
        while i < (address + data.len()) {
            let page_offset = i % 32;
            self.write_raw_blocking(
                i,
                &data[(i - address)..(i - address + (32 - page_offset)).min(data.len())],
                delay,
            )?;
            i += 32 - page_offset;
        }
        Ok(())
    }

    /// Read an arbitrary number of bytes from the EEPROM, starting at `address`.
    /// This function will automatically paginate.
    pub fn read_blocking(&mut self, address: usize, data: &mut [u8]) -> Result<(), E> {
        // No need to do this per-page
        // self.read_raw(address, data)

        #[cfg(feature = "defmt")]
        defmt::info!(
            "[EEPROM M24C64 Unit {:b}] Reading from address {:#x} with {} bytes",
            self.e_addr,
            address,
            data.len()
        );

        // Chunk the read into pages
        let len = data.len();
        let mut i = address;
        while i < (address + data.len()) {
            let page_offset = i % 32;
            self.read_raw_blocking(
                i,
                &mut data[(i - address)..(i - address + (32 - page_offset)).min(len)],
            )?;
            i += 32 - page_offset;
        }
        Ok(())
    }
}
