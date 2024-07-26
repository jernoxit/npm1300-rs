#![no_std]

use embedded_hal_async::digital::Wait;
use embedded_hal_async::i2c::Error as I2CHALError;
use embedded_hal_async::i2c::ErrorKind;

mod buck;
mod charger;
mod errlog;
mod fmt;
mod gpio;
mod interrupt;
mod ldo;
mod leddrv;
mod pof;
mod registers;
mod ship_hibernation;
mod sysmon;
mod sysreg;
mod timer;

const NRF1300_SLAVE_ADDRESS: u8 = 0b0110_1011;

enum Error {
    I2CError(ErrorKind),
    NoPinAssigned,
}
#[non_exhaustive]
enum InterruptSource {
    ADC,
    UNKNOWN,
}

struct NRF1300<I2C, SHPHLD, INTERRUPT>
where
    I2C: embedded_hal_async::i2c::I2c,
    SHPHLD: embedded_hal::digital::OutputPin,
    INTERRUPT: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
{
    i2c: I2C,
    shphld: Option<SHPHLD>,
    interrupt: Option<INTERRUPT>,
}

impl<I2C, SHPHLD, INTERRUPT> NRF1300<I2C, SHPHLD, INTERRUPT>
where
    I2C: embedded_hal_async::i2c::I2c,
    SHPHLD: embedded_hal::digital::OutputPin,
    INTERRUPT: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
{
    pub async fn new(i2c: I2C, shphld: Option<SHPHLD>, interrupt: Option<INTERRUPT>) -> Self {
        let mut nrf1300 = NRF1300 {
            i2c,
            shphld,
            interrupt,
        };

        nrf1300
    }

    async fn write_register(&mut self, register: u16, value: u8) -> Result<(), Error> {
        let reg_addr = register.to_be_bytes();
        let data: [u8; 3] = [reg_addr[0], reg_addr[1], value];

        match self.i2c.write(NRF1300_SLAVE_ADDRESS, &data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::I2CError(e.kind())),
        }
    }

    async fn read_register(&mut self, register: u16) -> Result<u8, Error> {
        let mut data = [0];
        match self
            .i2c
            .write_read(NRF1300_SLAVE_ADDRESS, &register.to_be_bytes(), &mut data)
            .await
        {
            Ok(_) => Ok(data[0]),
            Err(e) => Err(Error::I2CError(e.kind())),
        }
    }

    pub async fn wait_for_interrupt(&mut self) -> Result<InterruptSource, Error> {
        match self.interrupt {
            Some(ref mut interrupt) => {
                let _ = interrupt.wait_for_high().await;
                self.get_interrupt_source().await
            }
            None => Err(Error::NoPinAssigned),
        }
    }

    async fn get_interrupt_source(&mut self) -> Result<InterruptSource, Error> {
        return Ok(InterruptSource::UNKNOWN);
        todo!("Implement this function")
    }
}
