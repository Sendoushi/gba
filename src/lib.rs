#![no_std]
#![warn(missing_docs)]

//! This crate helps you write GBA ROMs.
//!
//! # SAFETY POLICY
//!
//! Some parts of this crate are safe wrappers around unsafe operations. This is
//! good, and what you'd expect from a Rust crate.
//!
//! However, the safe wrappers all assume that you will _only_ attempt to
//! execute this crate on a GBA or in a GBA Emulator.
//!
//! **Do not** use this crate in programs that aren't running on the GBA. If you
//! do, it's a giant bag of Undefined Behavior.

pub mod core_extras;
pub(crate) use crate::core_extras::*;

pub mod io_registers;

pub mod video_ram;

/// Combines the Red, Blue, and Green provided into a single color value.
pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}
