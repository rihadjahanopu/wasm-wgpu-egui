use std::collections::VecDeque;

/// Cross-platform high-precision time provider and frame rate monitor.
pub struct TimeTracker {
    last_instant: f64,
    delta_time: f32,
    fps: f32,
    frame_count: u64,
    frame_history: VecDeque<f32>,
    history_capacity: usize,
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeTracker {
    pub fn new() -> Self {
        let now = Self::get_timestamp_secs();
        let mut history = VecDeque::with_capacity(120);
        history.push_back(16.6);

        Self {
            last_instant: now,
            delta_time: 0.0166,
            fps: 60.0,
            frame_count: 0,
            frame_history: history,
            history_capacity: 120,
        }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Self::get_timestamp_secs();
        let raw_dt = (now - self.last_instant).max(0.0001);
        self.last_instant = now;
        self.delta_time = raw_dt as f32;
        self.frame_count += 1;

        // Exponential smoothing for FPS
        let instant_fps = 1.0 / self.delta_time;
        self.fps = self.fps * 0.92 + instant_fps * 0.08;

        let frame_ms = self.delta_time * 1000.0;
        if self.frame_history.len() >= self.history_capacity {
            self.frame_history.pop_front();
        }
        self.frame_history.push_back(frame_ms);

        self.delta_time
    }

    #[inline]
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    #[inline]
    pub fn fps(&self) -> f32 {
        self.fps
    }

    #[inline]
    pub fn frame_time_ms(&self) -> f32 {
        self.delta_time * 1000.0
    }

    #[inline]
    pub fn frame_history(&self) -> &VecDeque<f32> {
        &self.frame_history
    }

    #[inline]
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn get_timestamp_secs() -> f64 {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.performance())
                .map(|p| p.now() / 1000.0)
                .unwrap_or(0.0)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs_f64())
                .unwrap_or(0.0)
        }
    }
}
