pub(crate) mod spin {
    /// A simple spinlock with exponential backoff.
    pub type RawSpinMutex = spinning_top::RawSpinlock<spinning_top::relax::Backoff>;

    /// A [`lock_api::Mutex`] based on [`RawSpinMutex`].
    pub type SpinMutex<T> = lock_api::Mutex<RawSpinMutex, T>;

    /// A [`lock_api::MutexGuard`] based on [`RawSpinMutex`].
    pub type SpinMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawSpinMutex, T>;
}
pub(crate) mod ticket;

use interrupt_mutex::RawInterruptMutex;
use spin::RawSpinMutex;
use ticket::RawTicketMutex;

/// An interrupt-safe [`RawSpinMutex`].
pub type RawInterruptSpinMutex = RawInterruptMutex<RawSpinMutex>;

/// A [`lock_api::Mutex`] based on [`RawInterruptSpinMutex`].
pub type InterruptSpinMutex<T> = lock_api::Mutex<RawInterruptSpinMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptSpinMutex`].
pub type InterruptSpinMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawInterruptSpinMutex, T>;

/// An interrupt-safe [`RawTicketMutex`].
pub type RawInterruptTicketMutex = RawInterruptMutex<RawTicketMutex>;

/// A [`lock_api::Mutex`] based on [`RawInterruptTicketMutex`].
pub type InterruptTicketMutex<T> = lock_api::Mutex<RawInterruptTicketMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptTicketMutex`].
pub type InterruptTicketMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawInterruptTicketMutex, T>;
