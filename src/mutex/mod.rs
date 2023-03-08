pub(crate) mod interrupt;
#[cfg(not(feature = "all-still"))]
pub(crate) mod spin;
#[cfg(feature = "all-still")]
pub(crate) mod spin {
    pub use super::still::{
        RawStillMutex as RawSpinMutex, StillMutex as SpinMutex, StillMutexGuard as SpinMutexGuard,
    };
}
pub(crate) mod still;
#[cfg(not(feature = "all-still"))]
pub(crate) mod ticket;
#[cfg(feature = "all-still")]
pub(crate) mod ticket {
    pub use super::still::{
        RawStillMutex as RawTicketMutex, StillMutex as TicketMutex,
        StillMutexGuard as TicketMutexGuard,
    };
}

use interrupt::RawInterruptMutex;
use spin::RawSpinMutex;
use still::RawStillMutex;
use ticket::RawTicketMutex;

/// An interrupt-safe [`RawSpinMutex`].
pub type RawInterruptSpinMutex = RawInterruptMutex<RawSpinMutex>;

/// A [`lock_api::Mutex`] based on [`RawInterruptSpinMutex`].
pub type InterruptSpinMutex<T> = lock_api::Mutex<RawInterruptSpinMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptSpinMutex`].
pub type InterruptSpinMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawInterruptSpinMutex, T>;

/// An interrupt-safe [`RawStillMutex`].
pub type RawInterruptStillMutex = RawInterruptMutex<RawStillMutex>;

/// A [`lock_api::Mutex`] based on [`RawInterruptStillMutex`].
pub type InterruptStillMutex<T> = lock_api::Mutex<RawInterruptStillMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptStillMutex`].
pub type InterruptStillMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawInterruptStillMutex, T>;

/// An interrupt-safe [`RawTicketMutex`].
pub type RawInterruptTicketMutex = RawInterruptMutex<RawTicketMutex>;

/// A [`lock_api::Mutex`] based on [`RawInterruptTicketMutex`].
pub type InterruptTicketMutex<T> = lock_api::Mutex<RawInterruptTicketMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptTicketMutex`].
pub type InterruptTicketMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawInterruptTicketMutex, T>;
