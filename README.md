# Adafruit STEMMA soil moisture sensor &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/STEMMA_soil_moisture_sensor.svg

[crates.io]: https://crates.io/crates/STEMMA_soil_moisture_sensor
A pure generic I2C crate for the Adafruit STEMMA soil moisture sensor

## Usage

```rust
use stemma_soil_moisture_sensor::prelude::*;

fn main() -> Result<(), SoilMoistureSensorError> {
    // Setup your I2C and import relevant delay
    let i2c = ...;

    let moisture = SoilSensor::new(i2c, Delay).with_units(TemperatureUnit::Fahrenheit);
    let temp = moisture.temperature()?;
    let moist = moisture.moisture()?;
}
```