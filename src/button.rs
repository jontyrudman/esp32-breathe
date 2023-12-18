use core::cell::RefCell;
use critical_section::Mutex;
use esp_println::println;
use hal::{gpio, interrupt, peripherals};

#[allow(dead_code)]
pub enum Buttons<'a> {
    B0(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 0>>),
    B1(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 1>>),
    B2(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 2>>),
    B3(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 3>>),
    B4(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 4>>),
    B5(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 5>>),
    B6(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 6>>),
    B7(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 7>>),
    B8(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 8>>),
    B9(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 9>>),
    B10(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 10>>),
    B11(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 11>>),
    B12(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 12>>),
    B13(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 13>>),
    B14(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 14>>),
    B15(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 15>>),
    B16(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 16>>),
    B17(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 17>>),
    B18(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 18>>),
    B19(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 19>>),
    B20(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 20>>),
    B21(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 21>>),
    B22(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 22>>),
    B23(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 23>>),
    B24(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 24>>),
    B25(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 25>>),
    B26(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 26>>),
    B27(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 27>>),
    B32(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 32>>),
    B33(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 33>>),
    B34(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 34>>),
    B35(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 35>>),
    B36(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 36>>),
    B37(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 37>>),
    B38(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 38>>),
    B39(Button<'a, gpio::GpioPin<gpio::Input<gpio::PullDown>, 39>>),
}

pub struct Button<'a, GpioPin> {
    name: &'a str,
    pin: Mutex<RefCell<GpioPin>>,
}

impl<'a, GpioPin> Button<'a, GpioPin>
where
    GpioPin: gpio::Pin,
{
    pub fn new(name: &'a str, gpio_pin: GpioPin) -> Self {
        let btn = Button {
            name,
            pin: Mutex::new(RefCell::new(gpio_pin)),
        };

        critical_section::with(|cs| {
            btn.pin.borrow_ref_mut(cs).listen(gpio::Event::FallingEdge);
        });
        interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority2)
            .unwrap();

        return btn;
    }
}

pub trait Interruptor {
    fn isr(&self);
}

impl Interruptor for Buttons<'_> {
    fn isr(&self) {
        use Buttons::*;
        match self {
            B0(btn) => btn.isr(),
            B1(btn) => btn.isr(),
            B2(btn) => btn.isr(),
            B3(btn) => btn.isr(),
            B4(btn) => btn.isr(),
            B5(btn) => btn.isr(),
            B6(btn) => btn.isr(),
            B7(btn) => btn.isr(),
            B8(btn) => btn.isr(),
            B9(btn) => btn.isr(),
            B10(btn) => btn.isr(),
            B11(btn) => btn.isr(),
            B12(btn) => btn.isr(),
            B13(btn) => btn.isr(),
            B14(btn) => btn.isr(),
            B15(btn) => btn.isr(),
            B16(btn) => btn.isr(),
            B17(btn) => btn.isr(),
            B18(btn) => btn.isr(),
            B19(btn) => btn.isr(),
            B20(btn) => btn.isr(),
            B21(btn) => btn.isr(),
            B22(btn) => btn.isr(),
            B23(btn) => btn.isr(),
            B24(btn) => btn.isr(),
            B25(btn) => btn.isr(),
            B26(btn) => btn.isr(),
            B27(btn) => btn.isr(),
            B32(btn) => btn.isr(),
            B33(btn) => btn.isr(),
            B34(btn) => btn.isr(),
            B35(btn) => btn.isr(),
            B36(btn) => btn.isr(),
            B37(btn) => btn.isr(),
            B38(btn) => btn.isr(),
            B39(btn) => btn.isr(),
        }
    }
}

impl<GpioPin> Interruptor for Button<'_, GpioPin>
where
    GpioPin: gpio::Pin,
{
    fn isr(&self) {
        critical_section::with(|cs| {
            let mut pin = self.pin.borrow_ref_mut(cs);
            if pin.is_interrupt_set() {
                println!("Button {} pressed!", self.name);
                pin.clear_interrupt();
            }
        });
    }
}
