//! IMU sample types, shared between the firmware and the desktop app.
//!
//! These are raw register units as they come off the MPU6050. Converting them
//! to physical units is the firmware's job — see `firmware/src/imu_math.rs`.

/// Accel, temperature, and gyro registers are contiguous starting at
/// `ACCEL_OUT_H`, so one burst read picks up all 7 samples.
pub const READ_BUF_SIZE: usize = 14;

/// One full sample set, in raw register units.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImuData {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub temp_raw: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
}

impl ImuData {
    /// Decodes a 14-byte burst read starting at `ACCEL_OUT_H`.
    ///
    /// Register order is accel X/Y/Z, temperature, then gyro X/Y/Z — two bytes
    /// each, high byte first.
    pub fn from_bytes(buf: &[u8; READ_BUF_SIZE]) -> Self {
        todo!()
    }
}
