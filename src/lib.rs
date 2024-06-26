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
//! ## Examples
//!
//! ```
//! use hermit_sync::InterruptSpinMutex;
//!
//! static NUMBER: InterruptSpinMutex<usize> = InterruptSpinMutex::new(0);
//!
//! // Modify the data
//! *NUMBER.lock() = 2;
//!
//! // Read the data
//! let answer = *NUMBER.lock();
//! assert_eq!(2, answer);
//! ```
//!
//! # Initializing Static Data
//!
//! There are two primitives for safely initializing static data based on [`generic_once_cell`] and [`RawSpinMutex`]:
//! * [`OnceCell`] can be written to only once and can then be accessed without locking.
//! * [`Lazy`] wraps a [`OnceCell`] and is initialized on the first access from a closure.
//!
//! For API documentation see [`generic_once_cell::OnceCell`] and [`generic_once_cell::Lazy`].
//!
//! ## Examples
//!
//! ```
//! use std::collections::HashMap;
//!
//! use hermit_sync::InterruptLazy;
//!
//! static MAP: InterruptLazy<HashMap<usize, String>> = InterruptLazy::new(|| {
//!     // This is run on the first access of MAP.
//!     let mut map = HashMap::new();
//!     map.insert(42, "Ferris".to_string());
//!     map.insert(3, "やれやれだぜ".to_string());
//!     map
//! });
//!
//! assert_eq!("Ferris", MAP.get(&42).unwrap());
//! ```
//!
//! # Accessing Static Data Mutably
//!
//! There is [`ExclusiveCell`] for safely accessing static data mutable _once_.
//!
//! # Type Definitions
//!
//! This crate provides a lot of type definitions for ease of use:
//!
//! | [`RawMutex`]        | Base                  | With [`RawInterruptMutex`]     |
//! | ------------------- | --------------------- | ------------------------------ |
//! | `R`                 | [`Mutex`]             | [`InterruptMutex`]             |
//! | [`RawSpinMutex`]    |                       | [`RawInterruptSpinMutex`]      |
//! |                     | [`SpinMutex`]         | [`InterruptSpinMutex`]         |
//! |                     | [`SpinMutexGuard`]    | [`InterruptSpinMutexGuard`]    |
//! |                     | [`OnceCell`]          | [`InterruptOnceCell`]          |
//! |                     | [`Lazy`]              | [`InterruptLazy`]              |
//! | [`RawOneShotMutex`] |                       | [`RawInterruptOneShotMutex`]   |
//! |                     | [`OneShotMutex`]      | [`InterruptOneShotMutex`]      |
//! |                     | [`OneShotMutexGuard`] | [`InterruptOneShotMutexGuard`] |
//! | [`RawTicketMutex`]  |                       | [`RawInterruptTicketMutex`]    |
//! |                     | [`TicketMutex`]       | [`InterruptTicketMutex`]       |
//! |                     | [`TicketMutexGuard`]  | [`InterruptTicketMutexGuard`]  |
//!
//! [`RawMutex`]: lock_api::RawMutex
//! [`Mutex`]: lock_api::Mutex

#![cfg_attr(not(test), no_std)]
#![warn(unsafe_op_in_unsafe_fn)]

pub(crate) mod mutex;
#[cfg(not(feature = "all-one-shot"))]
pub(crate) mod rwlock {
    /// A simple spinning, read-preferring readers-writer lock with exponential backoff.
    pub type RawRwSpinLock = spinning_top::RawRwSpinlock<spinning_top::relax::Backoff>;

    /// A [`lock_api::RwLock`] based on [`RawRwSpinLock`].
    pub type RwSpinLock<T> = lock_api::RwLock<RawRwSpinLock, T>;

    /// A [`lock_api::RwLockReadGuard`] based on [`RawRwSpinLock`].
    pub type RwSpinLockReadGuard<'a, T> = lock_api::RwLockReadGuard<'a, RawRwSpinLock, T>;

    /// A [`lock_api::RwLockUpgradableReadGuard`] based on [`RawRwSpinLock`].
    pub type RwSpinLockUpgradableReadGuard<'a, T> =
        lock_api::RwLockUpgradableReadGuard<'a, RawRwSpinLock, T>;

    /// A [`lock_api::RwLockWriteGuard`] based on [`RawRwSpinLock`].
    pub type RwSpinLockWriteGuard<'a, T> = lock_api::RwLockWriteGuard<'a, RawRwSpinLock, T>;
}
#[cfg(feature = "all-one-shot")]
pub(crate) mod rwlock {
    pub use one_shot_mutex::{
        OneShotRwLock as RwSpinLock, OneShotRwLockReadGuard as RwSpinLockReadGuard,
        OneShotRwLockUpgradableReadGuard as RwSpinLockUpgradableReadGuard,
        OneShotRwLockWriteGuard as RwSpinLockWriteGuard, RawOneShotRwLock as RawRwSpinLock,
    };
}

pub use exclusive_cell::{CallOnce, CallOnceError, ExclusiveCell};
pub use interrupt_mutex::{InterruptMutex, InterruptMutexGuard, RawInterruptMutex};
pub use interrupts::without as without_interrupts;
pub use mutex::spin::{RawSpinMutex, SpinMutex, SpinMutexGuard};
pub use mutex::ticket::{RawTicketMutex, TicketMutex, TicketMutexGuard};
pub use mutex::{
    InterruptOneShotMutex, InterruptOneShotMutexGuard, InterruptSpinMutex, InterruptSpinMutexGuard,
    InterruptTicketMutex, InterruptTicketMutexGuard, RawInterruptOneShotMutex,
    RawInterruptSpinMutex, RawInterruptTicketMutex,
};
pub use one_shot_mutex::{
    OneShotMutex, OneShotMutexGuard, OneShotRwLock, OneShotRwLockReadGuard,
    OneShotRwLockUpgradableReadGuard, OneShotRwLockWriteGuard, RawOneShotMutex, RawOneShotRwLock,
};
pub use rwlock::{
    RawRwSpinLock, RwSpinLock, RwSpinLockReadGuard, RwSpinLockUpgradableReadGuard,
    RwSpinLockWriteGuard,
};

/// A [`generic_once_cell::OnceCell`], initialized using [`RawSpinMutex`].
pub type OnceCell<T> = generic_once_cell::OnceCell<RawSpinMutex, T>;

/// A [`generic_once_cell::Lazy`], initialized using [`RawSpinMutex`].
pub type Lazy<T, F = fn() -> T> = generic_once_cell::Lazy<RawSpinMutex, T, F>;

/// A [`generic_once_cell::OnceCell`], initialized using [`RawInterruptSpinMutex`].
pub type InterruptOnceCell<T> = generic_once_cell::OnceCell<RawInterruptSpinMutex, T>;

/// A [`generic_once_cell::Lazy`], initialized using [`RawInterruptSpinMutex`].
pub type InterruptLazy<T, F = fn() -> T> = generic_once_cell::Lazy<RawInterruptSpinMutex, T, F>;
