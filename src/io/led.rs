use esp_backtrace as _;
use hal::{gpio, ledc, prelude::*};

use crate::constants;

pub struct Led<'a, S, O>
where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
{
    pub channel: Option<ledc::channel::Channel<'a, S, O>>,
    pub ledc: Option<&'a ledc::LEDC<'a>>,
}

impl<'a, S, O> Led<'a, S, O>
where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
{
    pub fn new(ledc: &'a ledc::LEDC) -> Self {
        Led {
            channel: None,
            ledc: Some(ledc),
        }
    }
}

pub struct BreathingLed<'a, S, O> where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
{
    pub led: Led<'a, S, O>,
    pub min_duty: u8,
    pub max_duty: u8,
    pub breathe_in_time_ms: u16,
    pub breathe_out_time_ms: u16,
}

impl<'a, S, O> BreathingLed<'a, S, O>
where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
{
    pub fn new(ledc: &'a ledc::LEDC) -> Self {
        BreathingLed {
            led: Led::new(ledc),
            min_duty: 0,
            max_duty: 100,
            breathe_in_time_ms: constants::MIN_INHALE_TIME_MS,
            breathe_out_time_ms: constants::MIN_EXHALE_TIME_MS,
        }
    }
}

// TODO: One, non-blocking, breathing routine
pub trait Breather {
    fn breathe_in(&self);
    fn breathe_out(&self);
}

impl<'a, S, O> Breather for Led<'a, S, O>
where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
    ledc::channel::Channel<'a, S, O>: ledc::channel::ChannelHW<O>,
{
    fn breathe_in(&self) {
        self.channel
            .as_ref()
            .unwrap()
            .start_duty_fade(0, 100, 1000)
            .unwrap();
        while self.channel.as_ref().unwrap().is_duty_fade_running() {}
    }

    fn breathe_out(&self) {
        self.channel
            .as_ref()
            .unwrap()
            .start_duty_fade(100, 0, 1000)
            .unwrap();
        while self.channel.as_ref().unwrap().is_duty_fade_running() {}
    }
}

impl<'a, S, O> Breather for BreathingLed<'a, S, O>
where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
    ledc::channel::Channel<'a, S, O>: ledc::channel::ChannelHW<O>,
{
    fn breathe_in(&self) {
        self.led.channel
            .as_ref()
            .unwrap()
            .start_duty_fade(self.min_duty, self.max_duty, self.breathe_in_time_ms)
            .unwrap();
        while self.led.channel.as_ref().unwrap().is_duty_fade_running() {}
    }

    fn breathe_out(&self) {
        self.led.channel
            .as_ref()
            .unwrap()
            .start_duty_fade(self.max_duty, self.min_duty, self.breathe_out_time_ms)
            .unwrap();
        while self.led.channel.as_ref().unwrap().is_duty_fade_running() {}
    }
}
