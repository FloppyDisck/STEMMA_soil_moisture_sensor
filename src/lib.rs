#![no_std]

use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c;

pub mod error;
use crate::error::SoilMoistureSensorError;

pub mod prelude {
    pub use crate::error::SoilMoistureSensorError;
    pub use crate::{SoilSensor, TemperatureUnit};
}

const TEMP_C_CONSTANT: f32 = 0.000015258789;
const TEMP_F_CONSTANT: f32 = 0.000027466; // TEMP_C_CONSTANT * 1.8
const TEMP_F_CONSTANT_SUM: f32 = 32.0;

/// Influences what the reading temperature numbers are
#[derive(Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    #[default]
    Fahrenheit,
}

#[derive(Copy, Clone, Debug)]
pub struct SoilSensor<I2C: 'static + Send + Sync, D> {
    i2c: I2C,
    // Required for reading without getting any errors
    // https://github.com/adafruit/Adafruit_Seesaw/blob/8728936a5d1a0a7bf2887a82adb0828b70556a45/Adafruit_seesaw.cpp#L737
    delay: D,
    unit: TemperatureUnit,
    temp_delay: u16,
    moisture_delay: u16,
    address: u8,
}

impl<I2C, D> SoilSensor<I2C, D>
where
    I2C: 'static + Send + Sync,
    D: DelayUs<u16>,
{
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self {
            i2c,
            delay,
            unit: TemperatureUnit::Fahrenheit,
            temp_delay: 125,
            moisture_delay: 5000,
            address: 0x36,
        }
    }

    pub fn with_units(mut self, unit: TemperatureUnit) -> Self {
        self.unit = unit;
        self
    }

    /// Sets the address according to the enabled hardware settings
    pub fn with_address_pins(mut self, a0: bool, a1: bool) -> Self {
        self.address = 0x36;
        if a0 {
            self.address += 1;
        }
        if a1 {
            self.address += 2;
        }
        self
    }

    /// Sets the address
    pub fn with_address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    pub fn with_delay(mut self, temp: u16, moisture: u16) -> Self {
        self.temp_delay = temp;
        self.moisture_delay = moisture;
        self
    }
}

impl<I2C, D> SoilSensor<I2C, D>
where
    I2C: i2c::Write + i2c::Read + Send + Sync,
    D: DelayUs<u16>,
{
    pub fn temperature(&mut self) -> Result<f32, SoilMoistureSensorError> {
        let mut buffer = [0; 4];
        self.i2c_read(&[0x00, 0x04], &mut buffer, self.temp_delay)?;
        let raw = i32::from_be_bytes(buffer) as f32;
        Ok(match self.unit {
            TemperatureUnit::Celsius => raw * TEMP_C_CONSTANT,
            TemperatureUnit::Fahrenheit => (raw * TEMP_F_CONSTANT) + TEMP_F_CONSTANT_SUM,
        })
    }

    pub fn moisture(&mut self) -> Result<u16, SoilMoistureSensorError> {
        let mut buffer = [0; 2];
        self.i2c_read(&[0x0F, 0x10], &mut buffer, self.moisture_delay)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn i2c_read(
        &mut self,
        bytes: &[u8],
        buffer: &mut [u8],
        delay: u16,
    ) -> Result<(), SoilMoistureSensorError> {
        self.i2c
            .write(self.address, bytes)
            .map_err(|_| SoilMoistureSensorError::WriteI2CError)?;
        self.delay.delay_us(delay);
        self.i2c
            .read(self.address, buffer)
            .map_err(|_| SoilMoistureSensorError::ReadI2CError)
    }
}
