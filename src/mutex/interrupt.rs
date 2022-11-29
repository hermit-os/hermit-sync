use core::sync::atomic::Ordering;

use lock_api::{Mutex, MutexGuard, RawMutex};

use crate::interrupts::{self, AtomicFlags};

/// An interrupt-safe mutex.
///
/// This mutex wraps another [`RawMutex`] and disables interrupts while locked.
pub struct RawInterruptMutex<I> {
    inner: I,
    interrupt_flags: AtomicFlags,
}

unsafe impl<I: RawMutex> RawMutex for RawInterruptMutex<I> {
    const INIT: Self = Self {
        inner: I::INIT,
        interrupt_flags: AtomicFlags::new(interrupts::DISABLE),
    };

    type GuardMarker = I::GuardMarker;

    #[inline]
    fn lock(&self) {
        let interrupt_flags = crate::interrupts::read_disable();
        self.inner.lock();
        self.interrupt_flags
            .store(interrupt_flags, Ordering::Relaxed);
    }

    #[inline]
    fn try_lock(&self) -> bool {
        let interrupt_flags = crate::interrupts::read_disable();
        let ok = self.inner.try_lock();
        if !ok {
            crate::interrupts::restore(interrupt_flags);
        }
        ok
    }

    #[inline]
    unsafe fn unlock(&self) {
        let interrupt_flags = self
            .interrupt_flags
            .swap(interrupts::DISABLE, Ordering::Relaxed);
        unsafe {
            self.inner.unlock();
        }
        crate::interrupts::restore(interrupt_flags);
    }

    #[inline]
    fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }
}

/// A [`lock_api::Mutex`] based on [`RawInterruptMutex`].
pub type InterruptMutex<I, T> = Mutex<RawInterruptMutex<I>, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptMutex`].
pub type InterruptMutexGuard<'a, I, T> = MutexGuard<'a, RawInterruptMutex<I>, T>;
