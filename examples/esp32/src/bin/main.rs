#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use log::info;
use stemma_soil_moisture_sensor::prelude::*;

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    info!("Initializing Moisture Sensor");
    let i2c = I2c::new(
        peripherals.I2C0,
        Config::default()
    );
    let delay = Delay::new();
    let mut sensor = SoilSensor::new(i2c, delay).with_units(TemperatureUnit::Fahrenheit);
    
    loop {
        let reading = sensor.read().unwrap();
        info!("Temperature: {}\nMoisture: {}", reading.temperature, reading.moisture);
    }
}
