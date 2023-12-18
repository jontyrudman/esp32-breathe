use esp_backtrace as _;
use hal::{gpio, ledc, prelude::*};

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
