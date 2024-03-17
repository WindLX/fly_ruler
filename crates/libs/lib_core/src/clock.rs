use tokio::time::{sleep, Duration, Instant};

pub struct Clock {
    actual_start_time: Instant,
    pause_time: Instant,
    last_call: Instant,
    is_pause: bool,
    time_scale: f64,
    sample_time: Option<Duration>,
    listener_count: usize,
    listener_get_count: usize,
    now_virtual_time: Duration,
}

impl Clock {
    pub fn new(sample_time: Option<Duration>, time_scale: Option<f64>) -> Self {
        let actual_start_time = Instant::now();
        Self {
            actual_start_time,
            pause_time: actual_start_time,
            last_call: actual_start_time,
            is_pause: false,
            time_scale: time_scale.unwrap_or(1.0),
            sample_time,
            listener_count: 0,
            listener_get_count: 0,
            now_virtual_time: Duration::from_secs(0),
        }
    }

    /// start clock
    pub fn start(&mut self) {
        self.actual_start_time = Instant::now();
        self.pause_time = self.actual_start_time;
        self.last_call = self.actual_start_time;
        self.is_pause = false;
        self.now_virtual_time = Duration::from_secs(0);
    }

    /// reset clock
    pub fn reset(&mut self, time_scale: Option<f64>, sample_time: Option<Duration>) {
        self.actual_start_time = Instant::now();
        self.pause_time = self.actual_start_time;
        self.last_call = self.actual_start_time;
        self.is_pause = false;
        self.time_scale = time_scale.unwrap_or(1.0);
        self.sample_time = sample_time;
        self.now_virtual_time = Duration::from_secs(0);
    }

    pub fn subscribe(&mut self) {
        self.listener_count += 1;
    }

    pub fn unsubscribe(&mut self) {
        self.listener_count -= 1;
    }

    /// virtual current time
    pub async fn now(&mut self) -> Duration {
        if self.listener_count <= self.listener_get_count {
            self.listener_get_count = 0;
        }
        self.listener_get_count += 1;
        if self.is_pause {
            return (self.pause_time - self.actual_start_time).mul_f64(self.time_scale);
        }
        if self.listener_get_count == 1 {
            let time_scale = self.time_scale;
            let now = Instant::now();
            let virtual_delta_time = (now - self.actual_start_time).mul_f64(self.time_scale);

            if let Some(sample_time) = self.sample_time {
                let time_since_last_call = now - self.last_call;
                if time_since_last_call < sample_time {
                    let remaining_time = sample_time - time_since_last_call;
                    let remaining_time_f64 = remaining_time.as_secs_f64();

                    sleep(Duration::from_secs_f64(remaining_time_f64)).await;
                    let now = Instant::now();
                    let delta_time = now - self.actual_start_time;
                    self.last_call = now;
                    let v = delta_time.mul_f64(time_scale);
                    self.now_virtual_time = v;
                    return v;
                }
            }

            self.last_call = now;
            self.now_virtual_time = virtual_delta_time;
            return virtual_delta_time;
        } else {
            return self.now_virtual_time;
        }
    }

    // pause
    pub fn pause(&mut self) {
        if !self.is_pause {
            self.pause_time = Instant::now();
            self.is_pause = true;
        }
    }

    // resume
    pub fn resume(&mut self) {
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

#[cfg(test)]
mod core_clock_tests {
    use super::Clock;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_now() {
        let mut clock = Clock::new(Some(Duration::from_millis(1000)), None);
        clock.subscribe();
        clock.start();
        let r = clock.now().await;
        assert!(r.as_millis() - 1000 < 100);
        let r = clock.now().await;
        assert!(r.as_millis() - 2000 < 100);
    }

    #[tokio::test]
    async fn test_pause() {
        let mut clock = Clock::new(None, None);
        clock.subscribe();
        clock.start();
        clock.pause();
        tokio::time::sleep(Duration::from_secs(1)).await;
        clock.resume();
        let r = clock.now().await;
        assert_eq!(r.as_secs(), 0);
        tokio::time::sleep(Duration::from_secs(1)).await;
        let r = clock.now().await;
        assert_eq!(r.as_secs(), 1)
    }

    #[tokio::test]
    async fn test_scale() {
        let mut clock = Clock::new(None, Some(5.0));
        clock.subscribe();
        clock.start();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let r = clock.now().await;
        assert_eq!(r.as_secs(), 5)
    }

    #[tokio::test]
    async fn test_listener() {
        let mut clock = Clock::new(Some(Duration::from_millis(1000)), None);
        clock.subscribe();
        clock.subscribe();
        clock.start();
        let r = clock.now().await;
        let r_2 = clock.now().await;
        let r_3 = clock.now().await;
        assert_eq!(r.as_secs(), r_2.as_secs());
        assert!(r_3.as_millis() - r.as_millis() < 1100);
    }
}
