#[derive(Debug)]
pub struct Backoff {
    step: u8,
}

impl Backoff {
    const YIELD_LIMIT: u8 = 10;

    #[inline]
    pub fn new() -> Self {
        Backoff { step: 0 }
    }

    #[inline]
    pub fn snooze(&mut self) {
        for _ in 0..1_u16 << self.step {
            core::hint::spin_loop();
        }

        if !self.is_completed() {
            self.step += 1;
        }
    }

    #[inline]
    pub fn is_completed(&self) -> bool {
        self.step > Self::YIELD_LIMIT
    }
}

impl Default for Backoff {
    fn default() -> Backoff {
        Backoff::new()
    }
}
