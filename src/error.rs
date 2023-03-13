use thiserror::Error;

pub type Result<T> = std::result::Result<T, SoilMoistureSensorError>;

#[derive(Error, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SoilMoistureSensorError {
    #[error("Write Read I2C Error")]
    WriteReadI2CError,
    #[error("Write I2C Error")]
    WriteI2CError,
    #[error("Read I2C Error")]
    ReadI2CError,
}
