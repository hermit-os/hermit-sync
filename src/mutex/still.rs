use core::sync::atomic::{AtomicBool, Ordering};

use lock_api::{GuardSend, RawMutex};

/// An mutex without contention.
///
/// The opposite of [`RawSpinMutex`]: this mutex only spins once.
///
/// A simple mutex that panics on `lock` if the inner mutex is already locked.
/// This may be used on single-threaded configurations to panic on deadlocks.
///
/// [`RawSpinMutex`]: super::RawSpinMutex
pub struct RawStillMutex {
    lock: AtomicBool,
}

unsafe impl RawMutex for RawStillMutex {
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self = Self {
        lock: AtomicBool::new(false),
    };

    type GuardMarker = GuardSend;

    #[inline]
    fn lock(&self) {
        if !self.try_lock() {
            panic!("Attempted to lock a single threaded mutex that is locked.")
        }
    }

    #[inline]
    fn try_lock(&self) -> bool {
        self.lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    #[inline]
    unsafe fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }

    #[inline]
    fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
}

/// A [`lock_api::Mutex`] based on [`RawStillMutex`].
pub type StillMutex<T> = lock_api::Mutex<RawStillMutex, T>;

/// A [`lock_api::MutexGuard`] based on [`RawStillMutex`].
pub type StillMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawStillMutex, T>;
