use core::sync::atomic::{AtomicUsize, Ordering};

use crossbeam_utils::Backoff;
use lock_api::{GuardSend, Mutex, MutexGuard, RawMutex, RawMutexFair};

/// A [fair] [ticket lock] with [exponential backoff].
///
/// [fair]: https://en.wikipedia.org/wiki/Unbounded_nondeterminism
/// [ticket lock]: https://en.wikipedia.org/wiki/Ticket_lock
/// [exponential backoff]: https://en.wikipedia.org/wiki/Exponential_backoff
// Based on `spin::mutex::TicketMutex`, but with backoff.
pub struct RawTicketMutex {
    next_ticket: AtomicUsize,
    next_serving: AtomicUsize,
}

unsafe impl RawMutex for RawTicketMutex {
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self = Self {
        next_ticket: AtomicUsize::new(0),
        next_serving: AtomicUsize::new(0),
    };

    type GuardMarker = GuardSend;

    #[inline]
    fn lock(&self) {
        let ticket = self.next_ticket.fetch_add(1, Ordering::Relaxed);

        let backoff = Backoff::new();
        while self.next_serving.load(Ordering::Acquire) != ticket {
            backoff.spin();
        }
    }

    #[inline]
    fn try_lock(&self) -> bool {
        let ticket = self
            .next_ticket
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |ticket| {
                if self.next_serving.load(Ordering::Acquire) == ticket {
                    Some(ticket + 1)
                } else {
                    None
                }
            });

        ticket.is_ok()
    }

    #[inline]
    unsafe fn unlock(&self) {
        self.next_serving.fetch_add(1, Ordering::Release);
    }

    #[inline]
    fn is_locked(&self) -> bool {
        let ticket = self.next_ticket.load(Ordering::Relaxed);
        self.next_serving.load(Ordering::Relaxed) != ticket
    }
}

unsafe impl RawMutexFair for RawTicketMutex {
    #[inline]
    unsafe fn unlock_fair(&self) {
        unsafe { self.unlock() }
    }

    #[inline]
    unsafe fn bump(&self) {
        let ticket = self.next_ticket.load(Ordering::Relaxed);
        let serving = self.next_serving.load(Ordering::Relaxed);
        if serving + 1 != ticket {
            unsafe {
                self.unlock_fair();
                self.lock();
            }
        }
    }
}

pub type TicketMutex<T> = Mutex<RawTicketMutex, T>;
pub type TicketMutexGuard<'a, T> = MutexGuard<'a, RawTicketMutex, T>;

// From `spin::mutex::ticket`
#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;

    type TicketMutex<T> = super::TicketMutex<T>;

    #[derive(Eq, PartialEq, Debug)]
    struct NonCopy(i32);

    #[test]
    fn smoke() {
        let m = TicketMutex::<_>::new(());
        drop(m.lock());
        drop(m.lock());
    }

    #[test]
    fn lots_and_lots() {
        static M: TicketMutex<()> = TicketMutex::<_>::new(());
        static mut CNT: u32 = 0;
        const J: u32 = 1000;
        const K: u32 = 3;

        fn inc() {
            for _ in 0..J {
                unsafe {
                    let _g = M.lock();
                    CNT += 1;
                }
            }
        }

        let (tx, rx) = channel();
        for _ in 0..K {
            let tx2 = tx.clone();
            thread::spawn(move || {
                inc();
                tx2.send(()).unwrap();
            });
            let tx2 = tx.clone();
            thread::spawn(move || {
                inc();
                tx2.send(()).unwrap();
            });
        }

        drop(tx);
        for _ in 0..2 * K {
            rx.recv().unwrap();
        }
        assert_eq!(unsafe { CNT }, J * K * 2);
    }

    #[test]
    fn try_lock() {
        let mutex = TicketMutex::<_>::new(42);

        // First lock succeeds
        let a = mutex.try_lock();
        assert_eq!(a.as_ref().map(|r| **r), Some(42));

        // Additional lock failes
        let b = mutex.try_lock();
        assert!(b.is_none());

        // After dropping lock, it succeeds again
        ::core::mem::drop(a);
        let c = mutex.try_lock();
        assert_eq!(c.as_ref().map(|r| **r), Some(42));
    }

    #[test]
    fn test_into_inner() {
        let m = TicketMutex::<_>::new(NonCopy(10));
        assert_eq!(m.into_inner(), NonCopy(10));
    }

    #[test]
    fn test_into_inner_drop() {
        struct Foo(Arc<AtomicUsize>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let num_drops = Arc::new(AtomicUsize::new(0));
        let m = TicketMutex::<_>::new(Foo(num_drops.clone()));
        assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        {
            let _inner = m.into_inner();
            assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        }
        assert_eq!(num_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_mutex_arc_nested() {
        // Tests nested mutexes and access
        // to underlying data.
        let arc = Arc::new(TicketMutex::<_>::new(1));
        let arc2 = Arc::new(TicketMutex::<_>::new(arc));
        let (tx, rx) = channel();
        let _t = thread::spawn(move || {
            let lock = arc2.lock();
            let lock2 = lock.lock();
            assert_eq!(*lock2, 1);
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    fn test_mutex_arc_access_in_unwind() {
        let arc = Arc::new(TicketMutex::<_>::new(1));
        let arc2 = arc.clone();
        let _ = thread::spawn(move || -> () {
            struct Unwinder {
                i: Arc<TicketMutex<i32>>,
            }
            impl Drop for Unwinder {
                fn drop(&mut self) {
                    *self.i.lock() += 1;
                }
            }
            let _u = Unwinder { i: arc2 };
            panic!();
        })
        .join();
        let lock = arc.lock();
        assert_eq!(*lock, 2);
    }

    #[test]
    fn test_mutex_unsized() {
        let mutex: &TicketMutex<[i32]> = &TicketMutex::<_>::new([1, 2, 3]);
        {
            let b = &mut *mutex.lock();
            b[0] = 4;
            b[2] = 5;
        }
        let comp: &[i32] = &[4, 2, 5];
        assert_eq!(&*mutex.lock(), comp);
    }

    #[test]
    fn is_locked() {
        let mutex = TicketMutex::<_>::new(());
        assert!(!mutex.is_locked());
        let lock = mutex.lock();
        assert!(mutex.is_locked());
        drop(lock);
        assert!(!mutex.is_locked());
    }
}
