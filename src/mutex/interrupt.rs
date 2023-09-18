use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

use lock_api::{GuardNoSend, RawMutex};

/// An interrupt-safe mutex.
///
/// This mutex wraps another [`RawMutex`] and disables interrupts while locked.
/// Only has an effect if `target_os = "none"`.
pub struct RawInterruptMutex<I> {
    inner: I,
    interrupt_guard: UnsafeCell<MaybeUninit<interrupts::Guard>>,
}

// SAFETY: The `UnsafeCell` is locked by `inner`, initialized on `lock` and uninitialized on `unlock`.
unsafe impl<I: Sync> Sync for RawInterruptMutex<I> {}
// SAFETY: Mutexes cannot be send to other threads while locked.
// Sending them while unlocked is fine.
unsafe impl<I: Send> Send for RawInterruptMutex<I> {}

unsafe impl<I: RawMutex> RawMutex for RawInterruptMutex<I> {
    const INIT: Self = Self {
        inner: I::INIT,
        interrupt_guard: UnsafeCell::new(MaybeUninit::uninit()),
    };

    type GuardMarker = GuardNoSend;

    #[inline]
    fn lock(&self) {
        let guard = interrupts::disable();
        self.inner.lock();
        // SAFETY: We have exclusive access through locking `inner`.
        unsafe {
            self.interrupt_guard.get().write(MaybeUninit::new(guard));
        }
    }

    #[inline]
    fn try_lock(&self) -> bool {
        let guard = interrupts::disable();
        let ok = self.inner.try_lock();
        if ok {
            // SAFETY: We have exclusive access through locking `inner`.
            unsafe {
                self.interrupt_guard.get().write(MaybeUninit::new(guard));
            }
        }
        ok
    }

    #[inline]
    unsafe fn unlock(&self) {
        // SAFETY: We have exclusive access through locking `inner`.
        let guard = unsafe { self.interrupt_guard.get().replace(MaybeUninit::uninit()) };
        // SAFETY: `guard` was initialized when locking.
        let guard = unsafe { guard.assume_init() };
        unsafe {
            self.inner.unlock();
        }
        drop(guard);
    }

    #[inline]
    fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }
}

/// A [`lock_api::Mutex`] based on [`RawInterruptMutex`].
pub type InterruptMutex<I, T> = lock_api::Mutex<RawInterruptMutex<I>, T>;

/// A [`lock_api::MutexGuard`] based on [`RawInterruptMutex`].
pub type InterruptMutexGuard<'a, I, T> = lock_api::MutexGuard<'a, RawInterruptMutex<I>, T>;
