pub(crate) mod interrupt;
pub(crate) mod spin;
pub(crate) mod ticket;

use interrupt::{InterruptMutex, InterruptMutexGuard};
use spin::RawSpinMutex;
use ticket::RawTicketMutex;

pub(crate) type RawInterruptSpinMutex = interrupt::RawInterruptMutex<RawSpinMutex>;
pub type InterruptSpinMutex<T> = InterruptMutex<RawSpinMutex, T>;
pub type InterruptSpinMutexGuard<'a, T> = InterruptMutexGuard<'a, RawSpinMutex, T>;

pub type InterruptTicketMutex<T> = InterruptMutex<RawTicketMutex, T>;
pub type InterruptTicketMutexGuard<'a, T> = InterruptMutexGuard<'a, RawTicketMutex, T>;
