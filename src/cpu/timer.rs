use std::{
    num::NonZeroU8,
    time::{Duration, Instant},
};

pub struct Timers {
    delay_timer: u8,
    sound_timer: u8,
    last_instant: Instant,
}

pub struct Decremented {
    pub count: NonZeroU8,
    pub delay: bool,
    pub sound: bool,
}

impl Timers {
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            sound_timer: 0,
            last_instant: Instant::now(),
        }
    }

    pub fn set_delay_timer(&mut self, counter: u8) {
        self.delay_timer = counter;
    }

    pub fn set_sound_timer(&mut self, counter: u8) {
        self.sound_timer = counter;
    }

    pub fn count_down(&mut self) -> Option<Decremented> {
        let frame_length = Duration::from_secs_f64(1.0 / 60.0);
        let duration = self.last_instant.elapsed();

        // TODO: remove this panic
        let frames_skipped = (duration.as_nanos() / frame_length.as_nanos())
            .try_into()
            .expect("skipped more than 255 frames");

        let report = NonZeroU8::new(frames_skipped).map(|count| Decremented {
            count,
            delay: self.delay_timer > 0,
            sound: self.sound_timer > 0,
        });
        self.delay_timer = self.delay_timer.saturating_sub(frames_skipped);
        self.sound_timer = self.sound_timer.saturating_sub(frames_skipped);
        self.last_instant += frame_length * frames_skipped as u32;
        report
    }

    pub fn delay(&self) -> u8 {
        self.delay_timer
    }

    pub fn sound(&self) -> u8 {
        self.sound_timer
    }
}
