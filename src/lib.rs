//! # Overview
//!
//! hermit-sync provides synchronization primitives targeted at operating system kernels.
//!
//! # Interrupts
//!
//! [`without_interrupts`] runs a closure with disabled interrupts.
//!
//! # Mutexes
//!
//! This crate provides three kinds of mutexes based on [`lock_api::RawMutex`]:
//! * [`RawSpinMutex`] is a simple [test and test-and-set] [spinlock] with [exponential backoff].
//! * [`RawTicketMutex`] is a [fair] [ticket lock] with [exponential backoff].
//! * [`RawInterruptMutex`] wraps another mutex and disables interrupts while locked.
//!
//! [test and test-and-set]: https://en.wikipedia.org/wiki/Test_and_test-and-set
//! [spinlock]: https://en.wikipedia.org/wiki/Spinlock
//! [exponential backoff]: https://en.wikipedia.org/wiki/Exponential_backoff
//! [fair]: https://en.wikipedia.org/wiki/Unbounded_nondeterminism
//! [ticket lock]: https://en.wikipedia.org/wiki/Ticket_lock
//!
//! For API documentation see [`lock_api::Mutex`].
//!
//! # Initializing Static Data
//!
//! There are two primitives for safely initializing static data based on [`generic_once_cell`] and [`RawSpinMutex`]:
//! * [`OnceCell`] can be written to only once and can then be accessed without locking.
//! * [`Lazy`] wraps a [`OnceCell`] and is initialized on the first access from a closure.
//!
//! For API documentation see [`generic_once_cell::OnceCell`] and [`generic_once_cell::Lazy`].
//!
//! # Type Definitions
//!
//! This crate provides a lot of type definitions for ease of use:
//!
//! | [`RawMutex`]       | Base                 | With [`RawInterruptMutex`]    |
//! | ------------------ | -------------------- | ----------------------------- |
//! | [`RawSpinMutex`]   | [`SpinMutex`]        | [`InterruptSpinMutex`]        |
//! |                    | [`SpinMutexGuard`]   | [`InterruptSpinMutexGuard`]   |
//! |                    | [`OnceCell`]         | [`InterruptOnceCell`]         |
//! |                    | [`Lazy`]             | [`InterruptLazy`]             |
//! | [`RawTicketMutex`] | [`TicketMutex`]      | [`InterruptTicketMutex`]      |
//! |                    | [`TicketMutexGuard`] | [`InterruptTicketMutexGuard`] |
//!
//! [`RawMutex`]: lock_api::RawMutex

#![cfg_attr(not(test), no_std)]
#![warn(unsafe_op_in_unsafe_fn)]

pub(crate) mod interrupts;
pub(crate) mod mutex;

pub use interrupts::without_interrupts;
pub use mutex::{
    interrupt::{InterruptMutex, InterruptMutexGuard, RawInterruptMutex},
    spin::{RawSpinMutex, SpinMutex, SpinMutexGuard},
    ticket::{RawTicketMutex, TicketMutex, TicketMutexGuard},
    InterruptSpinMutex, InterruptSpinMutexGuard, InterruptTicketMutex, InterruptTicketMutexGuard,
};

pub type OnceCell<T> = generic_once_cell::OnceCell<mutex::spin::RawSpinMutex, T>;
pub type Lazy<T, F = fn() -> T> = generic_once_cell::Lazy<mutex::spin::RawSpinMutex, T, F>;

pub type InterruptOnceCell<T> = generic_once_cell::OnceCell<mutex::RawInterruptSpinMutex, T>;
pub type InterruptLazy<T, F = fn() -> T> =
    generic_once_cell::Lazy<mutex::RawInterruptSpinMutex, T, F>;
