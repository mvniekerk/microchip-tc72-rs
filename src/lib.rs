#![no_std]
use embedded_hal as hal;

use hal::blocking::spi;
use hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayMs;

pub enum Registers {
    Control = 0x00,
    Lsb = 0x01,
    Msb = 0x02,
    ManufacturerId = 0x03
}

pub struct Tc72<SPI, CS> {
    spi: SPI,
    cs: CS
}

const EXPECTED_ID: u8 = 0x54;

#[derive(Debug, PartialEq, Clone)]
pub enum Tc72Error<SpiError, PinError> {
    Spi(SpiError),
    Cs(PinError),
    ManufacturerWrong
}

impl<SPI, CS, SpiError, PinError> Tc72<SPI, CS>
    where
        SPI: spi::Transfer<u8, Error=SpiError> + spi::Write<u8, Error=SpiError>,
        CS: OutputPin<Error = PinError>
{
    /// Takes a config object to initialize the tc72 driver
    pub fn new(spi: SPI, cs: CS) -> Result<Self, Tc72Error<SpiError, PinError>> {
        let mut tc72 = Tc72 { spi, cs };
        let id = tc72.manufacturer_id()?;
        if id != EXPECTED_ID {
            Err(Tc72Error::ManufacturerWrong)
        } else {
            Ok(tc72)
        }
    }

    pub fn manufacturer_id(&mut self) -> Result<u8, Tc72Error<SpiError, PinError>> {
        self.read_reg(Registers::ManufacturerId)
    }

    pub fn control(&mut self, shutdown: bool, one_shot: bool) -> Result<(), Tc72Error<SpiError, PinError>>{
        let value = ((one_shot as u8) << 4) + (shutdown as u8);
        self.write_reg(Registers::Control, value)
    }

    pub fn temp_raw(&mut self) -> Result<u16, Tc72Error<SpiError, PinError>> {
        let msb = (self.read_reg(Registers::Msb)? as u16) << 2;
        let lsb = (self.read_reg(Registers::Lsb)? as u16) >> 6;
        Ok(msb | lsb)
    }

    pub fn temp(&mut self) -> Result<f32, Tc72Error<SpiError, PinError>> {
        self.temp_raw().map(|v| {
            let bit10: u16 = 1 << 9;
            // if bit 10 is set it is negative. To cast 10bit unsigned to 16bit signed move sign
            let v = if (v & bit10) > 0 { ( v & !bit10 ) | (1 << 15)} else { v };
            let v = v as i16;
            (v as f32) * 0.25f32
        } as f32)
    }

    pub fn one_shot_with_150ms_delay(&mut self, delay: &mut dyn DelayMs<u8>) -> Result<f32, Tc72Error<SpiError, PinError>> {
        self.control(false, true)?;
        delay.delay_ms(150);
        self.temp()
    }

    fn write_reg(&mut self, reg: Registers, value: u8) -> Result<(), Tc72Error<SpiError, PinError>>{
        let bytes = [((reg as u8) << 1)  | 0b0000_1000, value];
        self.cs.set_high().map_err(Tc72Error::Cs)?;
        self.spi.write(&bytes).map_err(Tc72Error::Spi)?;
        self.cs.set_low().map_err(Tc72Error::Cs)?;
        Ok(())
    }

    fn read_reg(&mut self, reg: Registers) -> Result<u8, Tc72Error<SpiError, PinError>> {
        let mut bytes = [((reg as u8) << 1), 0];
        self.cs.set_high().map_err(Tc72Error::Cs)?;
        self.spi.transfer(&mut bytes)
            .map_err(Tc72Error::Spi)?;
        self.cs.set_low().map_err(Tc72Error::Cs)?;
        Ok(bytes[1])
    }
}