#![no_std]
#![no_main]

mod button;
mod potentiometer;
mod led;

use esp_backtrace as _;
use esp_println::println;
use hal::{adc, analog, clock, gpio, ledc, peripherals, prelude::*};
use led::Breather;

const POT_PIN_NUM: u8 = 34;
const LED_PIN_NUM: u8 = 22;

type LedPinType = gpio::GpioPin<gpio::Output<gpio::PushPull>, LED_PIN_NUM>;
// type ButtonPinType = gpio::GpioPin<gpio::Input<gpio::PullDown>, BUTTON_PIN_NUM>;
type PotPinType = gpio::GpioPin<gpio::Analog, POT_PIN_NUM>;

static mut BUTTONS: [Option<button::Buttons>; 10] =
    [None, None, None, None, None, None, None, None, None, None];

const POT_READ_COUNT: u16 = 5;
const POT_MIN: u16 = 430;
const POT_MAX: u16 = 3410;
// Artificially hit the max and min segments later by expanding the deadzone
const POT_DEADZONE: u16 = 200;
const POT_SEGMENTS: u16 = 10;

#[entry]
fn main() -> ! {
    let peripherals = peripherals::Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let analog = peripherals.SENS.split();
    let clocks = clock::ClockControl::boot_defaults(system.clock_control).freeze();

    let io = gpio::IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Set up potentiometer
    let mut pot: potentiometer::Potentiometer<PotPinType, adc::ADC1> =
        potentiometer::Potentiometer::new();
    set_up_potentiometer(analog, io.pins.gpio34.into_analog(), &mut pot);

    // Set up button
    let btn: button::Buttons<'static> = button::Buttons::B15(button::Button::new(
        "Mode Selector",
        io.pins.gpio15.into_pull_down_input(),
    ));
    unsafe {
        BUTTONS[0] = Some(btn);
    }

    // LED setup
    let ledc = ledc::LEDC::new(peripherals.LEDC, &clocks);
    let mut hstimer = ledc.get_timer::<ledc::HighSpeed>(ledc::timer::Number::Timer0);
    let led = io.pins.gpio22.into_push_pull_output();
    let mut led_stuff: led::Led<
        ledc::HighSpeed,
        gpio::GpioPin<gpio::Output<gpio::PushPull>, LED_PIN_NUM>,
    > = led::Led::new(&ledc);
    set_up_ledc(led, &mut hstimer, &mut led_stuff);

    loop {
        let pot_value: &mut u16 = &mut 0;
        pot.read(pot_value);
        println!("Pot ADC reading = {}", pot_value);

        println!("Breathe in");
        led_stuff.breathe_in();

        println!("Breathe out");
        led_stuff.breathe_out();
    }
}


fn set_up_potentiometer(
    analog: analog::AvailableAnalog,
    gpio_pin: PotPinType,
    pot: &mut potentiometer::Potentiometer<PotPinType, adc::ADC1>,
) {
    // ADC instances for pot
    let mut adc1_config = adc::AdcConfig::new();
    critical_section::with(|cs| {
        pot.adc_pin
            .borrow_ref_mut(cs)
            .replace(adc1_config.enable_pin(gpio_pin, adc::Attenuation::Attenuation6dB))
    });
    critical_section::with(|cs| {
        pot.adc
            .borrow_ref_mut(cs)
            .replace(adc::ADC::<adc::ADC1>::adc(analog.adc1, adc1_config).unwrap())
    });

    pot.min_val = POT_MIN;
    pot.max_val = POT_MAX;
    pot.deadzone = POT_DEADZONE;
    pot.segments = POT_SEGMENTS;
    pot.read_count = POT_READ_COUNT;
}

fn set_up_ledc<'a>(
    pin: LedPinType,
    hstimer: &'a mut ledc::timer::Timer<ledc::HighSpeed>,
    led_stuff: &mut led::Led<'a, ledc::HighSpeed, LedPinType>,
) {
    let mut ch = led_stuff
        .ledc
        .unwrap()
        .get_channel(ledc::channel::Number::Channel0, pin);
    hstimer
        .configure(ledc::timer::config::Config {
            duty: ledc::timer::config::Duty::Duty5Bit,
            clock_source: ledc::timer::HSClockSource::APBClk,
            frequency: 24u32.kHz(),
        })
        .unwrap();

    ch.configure(ledc::channel::config::Config {
        timer: hstimer,
        duty_pct: 10,
        pin_config: ledc::channel::config::PinConfig::PushPull,
    })
    .unwrap();

    ch.start_duty_fade(0, 100, 2000).expect_err(
        "Fading from 0% to 100%, at 24kHz and 5-bit resolution, over 2 seconds, should fail",
    );

    led_stuff.channel = Some(ch);
}

// Turn off interrupt bits set on button press
// TODO: Extend to handle different button actions
#[hal::macros::ram]
#[interrupt]
unsafe fn GPIO() {
    use button::Interruptor;
    for btn_opt in &BUTTONS {
        match btn_opt {
            Some(btn) => btn.isr(),
            _ => {}
        }
    }
}
