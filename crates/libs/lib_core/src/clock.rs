use tokio::time::{Duration, Instant};

pub trait AsClock
where
    Self: Send,
{
    fn start(&mut self);
    fn now(&mut self) -> Duration;
    fn pause(&mut self);
    fn resume(&mut self);
}

pub struct Clock {
    actual_start_time: Instant,
    pause_time: Instant,
    last_call: Instant,
    is_pause: bool,
    now_virtual_time: Duration,
}

impl Clock {
    pub fn new() -> Self {
        let actual_start_time = Instant::now();
        Self {
            actual_start_time,
            pause_time: actual_start_time,
            last_call: actual_start_time,
            is_pause: false,
            now_virtual_time: Duration::from_secs(0),
        }
    }
}

impl AsClock for Clock {
    /// start clock
    fn start(&mut self) {
        self.actual_start_time = Instant::now();
        self.pause_time = self.actual_start_time;
        self.last_call = self.actual_start_time;
        self.is_pause = false;
        self.now_virtual_time = Duration::from_secs(0);
    }

    /// virtual current time
    fn now(&mut self) -> Duration {
        if self.is_pause {
            return self.pause_time - self.actual_start_time;
        }
        let now = Instant::now();
        let virtual_delta_time = now - self.actual_start_time;

        self.last_call = now;
        self.now_virtual_time = virtual_delta_time;
        return virtual_delta_time;
    }

    // pause
    fn pause(&mut self) {
        if !self.is_pause {
            self.pause_time = Instant::now();
            self.is_pause = true;
        }
    }

    // resume
    fn resume(&mut self) {
        if self.is_pause {
            let pause_duration = Instant::now() - self.pause_time;
            self.actual_start_time += pause_duration;
            self.last_call += pause_duration;
            self.pause_time = self.actual_start_time;
            self.is_pause = false;
        }
    }
}

unsafe impl Send for Clock {}

pub struct FixedClock {
    actual_start_time: Instant,
    last_call: Instant,
    is_pause: bool,
    time_scale: f64,
    sample_time: Duration,
}

impl FixedClock {
    pub fn new(sample_time: Duration, time_scale: Option<f64>) -> Self {
        let actual_start_time = Instant::now();
        Self {
            actual_start_time,
            last_call: actual_start_time,
            is_pause: false,
            time_scale: time_scale.unwrap_or(1.0),
            sample_time,
        }
    }
}

unsafe impl Send for FixedClock {}

impl AsClock for FixedClock {
    /// start clock
    fn start(&mut self) {
        self.actual_start_time = Instant::now();
        self.last_call = self.actual_start_time;
        self.is_pause = false;
    }

    /// virtual current time
    fn now(&mut self) -> Duration {
        if !self.is_pause {
            self.last_call += self.sample_time;
        }
        return (self.last_call - self.actual_start_time).mul_f64(self.time_scale);
    }

    // pause
    fn pause(&mut self) {
        if !self.is_pause {
            self.is_pause = true;
        }
    }

    // resume
    fn resume(&mut self) {
        if self.is_pause {
            self.is_pause = false;
        }
    }
}

#[cfg(test)]
mod core_clock_tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_now() {
        let mut clock = FixedClock::new(Duration::from_millis(1000), None);
        clock.start();
        let r = clock.now();
        assert!(r.as_millis() - 1000 < 100);
        let r = clock.now();
        assert!(r.as_millis() - 2000 < 100);
    }

    #[tokio::test]
    async fn test_pause() {
        let mut clock = FixedClock::new(Duration::from_millis(1000), None);
        clock.start();
        clock.pause();
        tokio::time::sleep(Duration::from_secs(1)).await;
        clock.resume();
        let r = clock.now();
        assert_eq!(r.as_secs(), 0);
        tokio::time::sleep(Duration::from_secs(1)).await;
        let r = clock.now();
        assert_eq!(r.as_secs(), 1)
    }

    #[tokio::test]
    async fn test_scale() {
        let mut clock = FixedClock::new(Duration::from_millis(1000), None);
        clock.start();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let r = clock.now();
        assert_eq!(r.as_secs(), 5)
    }
}
