use embassy_nrf::gpio;
use embassy_nrf::gpio::Pin;
use embassy_nrf::gpio::Pull;
use embedded_hal::digital::v2::InputPin;

pub struct Button<T: Pin> {
    pin: gpio::Input<'static, T>
}

impl<T: Pin> Button<T> {
    pub fn new(pin: T) -> Self {
        Self {
            pin: gpio::Input::new(pin, Pull::None)
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
}