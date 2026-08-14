//! Types shared between the ORT firmware and the desktop app.
//!
//! This crate is `no_std` and holds data types only — no hardware access and
//! no sensor math. Drivers live in `firmware/src/`, and the math that turns
//! these raw samples into physical units lives in `firmware/src/imu_math.rs`.
//!
//! Two optional features control the derives, so neither consumer pays for
//! the other's: `defmt` (firmware, RTT logging) and `serde` (desktop, JSON
//! across the Tauri boundary).

#![cfg_attr(not(feature = "specta"), no_std)]

pub mod gps;
pub mod imu;
mod parse_utils;

pub use gps::{NMEA_MAX_LEN, NmeaMessage};
pub use imu::{ImuData, READ_BUF_SIZE};
