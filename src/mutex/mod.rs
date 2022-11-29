pub(crate) mod interrupt;
pub(crate) mod spin;
pub(crate) mod ticket;

use lock_api::{Mutex, MutexGuard};

use interrupt::RawInterruptMutex;
use spin::RawSpinMutex;
use ticket::RawTicketMutex;

/// An interrupt-safe [`RawSpinMutex`].
pub type RawInterruptSpinMutex = RawInterruptMutex<RawSpinMutex>;

/// A [`lock_api::Mutex`] based on [`RawInterruptSpinMutex`].
pub type InterruptSpinMutex<T> = Mutex<RawInterruptSpinMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptSpinMutex`].
pub type InterruptSpinMutexGuard<'a, T> = MutexGuard<'a, RawInterruptSpinMutex, T>;

/// An interrupt-safe [`RawTicketMutex`].
pub type RawInterruptTicketMutex = RawInterruptMutex<RawTicketMutex>;

/// A [`lock_api::Mutex`] based on [`RawInterruptTicketMutex`].
pub type InterruptTicketMutex<T> = Mutex<RawInterruptTicketMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptTicketMutex`].
pub type InterruptTicketMutexGuard<'a, T> = MutexGuard<'a, RawInterruptTicketMutex, T>;
