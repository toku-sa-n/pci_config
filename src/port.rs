//! A module providing an interface to access I/O ports.
//!
//! This crate does not use the `in` and `out` assembly instructions directly because, in some cases
//! applications cannot execute them (e.g., they run in the user mode). Instead, this crate provides
//! the [`Accessor`] crate to access the I/O ports.

/// A trait to read from and write to the I/O ports.
pub trait Accessor {
    /// Reads a value from the I/O port `port`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `port` is the correct one. Reading from an arbitrary port may
    /// have side effects.
    unsafe fn inl(&mut self, port: u16) -> u32;

    /// Writes a `value` to the I/O port `port`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `port` is the correct one. Writing to an arbitrary port may
    /// have side effects.
    unsafe fn outl(&mut self, port: u16, value: u32);
}
