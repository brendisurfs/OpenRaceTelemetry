//! Types shared between the ORT firmware and the desktop app.
//!
//! This crate is `no_std` and holds data types only, no hardware access and
//! no sensor math. Drivers live in `firmware/src/`, and the math that turns
//! these raw samples into physical units lives in `firmware/src/imu_math.rs`.
//!
//! Two optional features control the derives, so neither consumer pays for
//! the other's: `defmt` (firmware, RTT logging) and `serde` (desktop, JSON
//! across the Tauri boundary).
//!
//! TypeScript codegen deliberately lives outside this crate: specta is
//! std-only, and the frontend wants strings where the wire format wants bytes.
//! Both concerns are handled by the DTOs in `desktop/src-tauri/src/dto.rs`.

#![cfg_attr(not(test), no_std)]

pub mod gps;
pub mod imu;

pub use gps::{MAX_NMEA_LEN, NmeaMessage};
pub use imu::{ImuData, READ_BUF_SIZE};
