use crate::M24C64;
use embedded_hal_async::{delay::DelayNs, i2c};

impl<I2C, E> M24C64<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    async fn write_raw<D: DelayNs>(
        &mut self,
        address: usize,
        bytes: &[u8],
        delay: &mut D,
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
            match self
                .i2c
                .write(self.e_addr | 0x50, &self.cmd_buf[0..bytes.len() + 2])
                .await
            {
                Ok(_) => return Ok(()),
                Err(_) if i < 10 => (),
                Err(e) => return Err(e),
            }
            i += 1;

            delay.delay_ms(1).await;
        }
    }

    async fn read_raw(&mut self, address: usize, bytes: &mut [u8]) -> Result<(), E> {
        self.cmd_buf[0] = (address >> 8) as u8;
        self.cmd_buf[1] = (address & 0xFF) as u8;

        self.i2c
            .write_read(self.e_addr | 0x50, &self.cmd_buf[0..2], bytes)
            .await
    }

    /// Write an arbitrary number of bytes into the EEPROM, starting at `address`.
    /// This function will automatically paginate.
    pub async fn write<D: DelayNs>(
        &mut self,
        address: usize,
        data: &[u8],
        delay: &mut D,
    ) -> Result<(), E> {
        // Chunk the write into pages
        let mut i = address;
        while i < (address + data.len()) {
            let page_offset = i % 32;
            self.write_raw(
                i,
                &data[(i - address)..(i - address + (32 - page_offset)).min(data.len())],
                delay,
            )
            .await?;
            i += 32 - page_offset;
        }
        Ok(())
    }

    /// Read an arbitrary number of bytes from the EEPROM, starting at `address`.
    /// This function will automatically paginate.
    pub async fn read(&mut self, address: usize, data: &mut [u8]) -> Result<(), E> {
        // No need to do this per-page
        // self.read_raw(address, data)

        // Chunk the read into pages
        let len = data.len();
        let mut i = address;
        while i < (address + data.len()) {
            let page_offset = i % 32;
            self.read_raw(
                i,
                &mut data[(i - address)..(i - address + (32 - page_offset)).min(len)],
            )
            .await?;
            i += 32 - page_offset;
        }
        Ok(())
    }
}
