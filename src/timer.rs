use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub struct Timer {
    ready: bool,
    millis: u64,
    duration: Duration,
}

impl Timer {
    pub fn from_millis(millis: u64) -> Self {
        Self {
            ready: false,
            duration: Duration::from_millis(millis),
            millis,
        }
    }

    pub fn update(&mut self, delta: Duration) -> &Self {
        self.duration = self.duration.saturating_sub(delta);
        self.ready = self.duration.is_zero();
        self
    }

    pub fn finish(&mut self) {
        self.duration = Duration::from_millis(0);
        self.ready = true;
    }

    #[inline]
    pub fn ready(&self) -> bool {
        self.ready
    }

    pub fn reset(&mut self) {
        *self = Timer::from_millis(self.millis);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_timer() {
        let mut timer = Timer::from_millis(1000);
        assert!(!timer.ready());
        timer.update(Duration::from_millis(500));
        assert!(!timer.ready());
        timer.update(Duration::from_millis(501));
        assert!(timer.ready());
        timer.reset();
        assert!(!timer.ready());
        timer.finish();
        assert!(timer.ready());
    }
}
