use core::{borrow::BorrowMut, cell::RefCell};
use critical_section::Mutex;

use hal::{adc, prelude::*};

pub struct Potentiometer<'a, GpioPin, Adc>
where
    Adc: adc::RegisterAccess,
    GpioPin: embedded_hal::adc::Channel<Adc, ID = u8>,
{
    pub read_count: u16,
    pub min_val: u16,
    pub max_val: u16,
    pub deadzone: u16,
    pub segments: u16,
    pub adc: Mutex<RefCell<Option<adc::ADC<'a, Adc>>>>,
    pub adc_pin: Mutex<RefCell<Option<adc::AdcPin<GpioPin, Adc>>>>,
}

impl<'a, GpioPin, Adc> Potentiometer<'a, GpioPin, Adc>
where
    Adc: adc::RegisterAccess,
    GpioPin: embedded_hal::adc::Channel<Adc, ID = u8>,
{
    pub fn new() -> Self {
        Potentiometer {
            read_count: 0,
            min_val: 0,
            max_val: 0,
            deadzone: 0,
            segments: 1,
            adc: Mutex::new(RefCell::new(None)),
            adc_pin: Mutex::new(RefCell::new(None)),
        }
    }

    pub fn read(&self, value: &mut u16) {
        critical_section::with(|cs| {
            let mut avg_value = 0;
            for _ in 0..self.read_count {
                let v = nb::block!(self.adc.borrow_ref_mut(cs).as_mut().unwrap().read(
                    &mut self
                        .adc_pin
                        .borrow_ref_mut(cs)
                        .borrow_mut()
                        .as_mut()
                        .unwrap()
                ))
                .unwrap();
                avg_value = avg_value + (v / (self.read_count + 1))
            }
            let augmented_max = self.max_val + self.deadzone;
            let augmented_min = self.min_val - self.deadzone;
            let bounded_value = match avg_value {
                x if x > augmented_max => augmented_max,
                x if x < augmented_min => augmented_min,
                x => x,
            };
            let divisor = (augmented_max - augmented_min) / self.segments;
            let segment = bounded_value / divisor;

            *value = segment;
        });
    }
}
