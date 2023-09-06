/// Run a closure with disabled interrupts.
///
/// Run the given closure, disabling interrupts before running it (if they aren't already disabled).
/// Afterwards, interrupts are enabling again if they were enabled before.
///
/// If you have other `enable` and `disable` calls _within_ the closure, things may not work as expected.
///
/// Only has an effect if `target_os = "none"`.
///
/// # Examples
///
/// ```
/// use hermit_sync::without_interrupts;
///
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
    if cfg!(target_os = "none") {
        let flags = imp::get();

        if flags != DISABLE {
            imp::set(DISABLE);
        }

        flags
    } else {
        DISABLE
    }
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

    /// Set the `A`, `I`, and `F` bit for _masking_ interrupts.
    pub const DISABLE: u64 = 0b111000000;

    #[inline]
    pub fn get() -> u64 {
        // Return only the relevant bits
        DAIF.get() & DISABLE
    }

    #[inline]
    pub fn set(value: u64) {
        // Set only the relevant bits
        let value = (DAIF.get() & !DISABLE) | value;
        DAIF.set(value);
    }
}

#[cfg(target_arch = "riscv64")]
mod imp {
    use riscv::register::sstatus;

    pub type Flags = bool;
    pub type AtomicFlags = core::sync::atomic::AtomicBool;

    pub const DISABLE: bool = false;

    #[inline]
    pub fn get() -> bool {
        sstatus::read().sie()
    }

    #[inline]
    pub fn set(value: bool) {
        unsafe {
            if value {
                sstatus::set_sie();
            } else {
                sstatus::clear_sie();
            }
        }
    }
}
