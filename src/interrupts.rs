/// Run a closure with disabled interrupts.
///
/// Run the given closure, disabling interrupts before running it (if they aren't already disabled).
/// Afterwards, interrupts are enabling again if they were enabled before.
///
/// If you have other `enable` and `disable` calls _within_ the closure, things may not work as expected.
///
/// # Examples
///
/// ```ignore
/// // interrupts are enabled
/// without_interrupts(|| {
///     // interrupts are disabled
///     without_interrupts(|| {
///         // interrupts are disabled
///     });
///     // interrupts are still disabled
/// });
/// // interrupts are enabled again
/// ```
// Doc taken from `x86_64::instructions::interrupts::without_interrupts`.
#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let flags = read_disable();

    let ret = f();

    restore(flags);

    ret
}

#[inline]
pub(crate) fn read_disable() -> imp::Flags {
    let flags = imp::get();

    if flags != DISABLE && cfg!(target_os = "none") {
        imp::set(DISABLE);
    }

    flags
}

#[inline]
pub(crate) fn restore(flags: imp::Flags) {
    if flags != DISABLE && cfg!(target_os = "none") {
        imp::set(flags);
    }
}

pub(crate) use imp::{AtomicFlags, DISABLE};

#[cfg(target_arch = "x86_64")]
mod imp {
    use x86_64::instructions::interrupts;

    pub type Flags = bool;
    pub type AtomicFlags = core::sync::atomic::AtomicBool;

    pub const DISABLE: bool = false;

    pub use interrupts::are_enabled as get;

    #[inline]
    pub fn set(enable: bool) {
        if enable {
            interrupts::enable();
        } else {
            interrupts::disable();
        }
    }
}

#[cfg(target_arch = "aarch64")]
mod imp {
    use aarch64_cpu::registers::DAIF;
    use tock_registers::interfaces::{Readable, Writeable};

    pub type Flags = u64;
    pub type AtomicFlags = core::sync::atomic::AtomicU64;

    pub const DISABLE: u64 = 0;

    #[inline]
    pub fn get() -> u64 {
        DAIF.get()
    }

    #[inline]
    pub fn set(value: u64) {
        DAIF.set(value);
    }
}
