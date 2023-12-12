#![no_std]
#![no_main]

use core::{borrow::BorrowMut, cell::RefCell};

use critical_section::Mutex;
use esp_backtrace as _;
use esp_println::println;
use hal::{adc, analog, clock, gpio, interrupt, ledc, peripherals, prelude::*};

const BUTTON_PIN_NUM: u8 = 15;
const POT_PIN_NUM: u8 = 34;
const LED_PIN_NUM: u8 = 22;

type LedPinType = gpio::GpioPin<gpio::Output<gpio::PushPull>, LED_PIN_NUM>;
type ButtonPinType = gpio::GpioPin<gpio::Input<gpio::PullDown>, BUTTON_PIN_NUM>;
type PotPinType = gpio::GpioPin<gpio::Analog, POT_PIN_NUM>;

static BUTTON: Mutex<RefCell<Option<ButtonPinType>>> =
    Mutex::new(RefCell::new(None));

static POT_ADCPIN: Mutex<
    RefCell<Option<adc::AdcPin<PotPinType, adc::ADC1>>>,
> = Mutex::new(RefCell::new(None));

static ADC1: Mutex<RefCell<Option<adc::ADC<'static, adc::ADC1>>>> = Mutex::new(RefCell::new(None));

struct Led<'a, S, O>
where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
{
    pub channel: Option<ledc::channel::Channel<'a, S, O>>,
    ledc: Option<&'a ledc::LEDC<'a>>,
}

impl<'a, S, O> Led<'a, S, O>
where
    S: ledc::timer::TimerSpeed,
    O: gpio::OutputPin,
{
    fn new(ledc: &'a ledc::LEDC) -> Self {
        Led {
            channel: None,
            ledc: Some(ledc),
        }
    }
}

trait Breather {
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
        println!("Breathe in");
        self.channel.as_ref().unwrap().start_duty_fade(0, 100, 1000).unwrap();
        while self.channel.as_ref().unwrap().is_duty_fade_running() {}
    }

    fn breathe_out(&self) {
        println!("Breathe out");
        self.channel.as_ref().unwrap().start_duty_fade(100, 0, 1000).unwrap();
        while self.channel.as_ref().unwrap().is_duty_fade_running() {}
    }
}

fn set_up_potentiometer(
    analog: analog::AvailableAnalog,
    gpio_pin: gpio::GpioPin<gpio::Analog, 34>,
) {
    // ADC instances for pot
    let mut adc1_config = adc::AdcConfig::new();
    critical_section::with(|cs| {
        POT_ADCPIN
            .borrow_ref_mut(cs)
            .replace(adc1_config.enable_pin(gpio_pin, adc::Attenuation::Attenuation6dB))
    });
    critical_section::with(|cs| {
        ADC1.borrow_ref_mut(cs)
            .replace(adc::ADC::<adc::ADC1>::adc(analog.adc1, adc1_config).unwrap())
    });
}

fn set_up_button(mut gpio_pin: ButtonPinType) {
    gpio_pin.listen(gpio::Event::FallingEdge);
    critical_section::with(|cs| BUTTON.borrow_ref_mut(cs).replace(gpio_pin));
    interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority2).unwrap();
}

fn set_up_ledc<'a>(pin: LedPinType, hstimer: &'a mut ledc::timer::Timer<ledc::HighSpeed>, led_stuff: &mut Led<'a, ledc::HighSpeed, LedPinType>) {
    let mut ch = led_stuff.ledc.unwrap().get_channel(ledc::channel::Number::Channel0, pin);
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

#[entry]
fn main() -> ! {
    let peripherals = peripherals::Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let analog = peripherals.SENS.split();
    let clocks = clock::ClockControl::boot_defaults(system.clock_control).freeze();

    let io = gpio::IO::new(peripherals.GPIO, peripherals.IO_MUX);

    set_up_potentiometer(analog, io.pins.gpio34.into_analog());
    set_up_button(io.pins.gpio15.into_pull_down_input());

    // LED setup
    let ledc = ledc::LEDC::new(peripherals.LEDC, &clocks);
    let mut hstimer = ledc.get_timer::<ledc::HighSpeed>(ledc::timer::Number::Timer0);
    let led = io.pins.gpio22.into_push_pull_output();
    let mut led_stuff: Led<
        ledc::HighSpeed,
        gpio::GpioPin<gpio::Output<gpio::PushPull>, LED_PIN_NUM>,
    > = Led::new(&ledc);
    set_up_ledc(led, &mut hstimer, &mut led_stuff);

    loop {
        pot_read();
        led_stuff.breathe_in();
        led_stuff.breathe_out();
    }
}

fn pot_read() {
    critical_section::with(|cs| {
        let pot_value: u16 = nb::block!(ADC1
            .borrow_ref_mut(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .read(&mut POT_ADCPIN.borrow_ref_mut(cs).borrow_mut().as_mut().unwrap()))
        .unwrap();
        println!("Pot ADC reading = {}", pot_value);
    });
}

#[hal::macros::ram]
#[interrupt]
unsafe fn GPIO() {
    println!("Button pressed!");
    critical_section::with(|cs| {
        BUTTON
            .borrow_ref_mut(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt();
    });
}
