#![no_std]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

use embassy::executor::SpawnError;
use embassy::executor::Spawner;
use embassy_nrf::gpio::NoPin;
use embassy_nrf::interrupt;
use embassy_nrf::peripherals::P0_14;
use embassy_nrf::peripherals::P0_23;

pub use embassy_nrf::Peripherals as RawPeripherals;

pub mod button;
pub mod display;

pub use button::Button;
pub use display::Display;
use embassy_nrf::peripherals::UARTE0;
use embassy_nrf::uarte;
use embassy_nrf::uarte::Baudrate;
use embassy_nrf::uarte::Parity;
use embassy_nrf::uarte::Uarte;

pub struct Peripherals {
    pub display: Display,
    pub button_a: Button<P0_14>,
    pub button_b: Button<P0_23>,
    pub uart: Uarte<'static, UARTE0>,
}

impl Peripherals {
    pub fn new(
        peripherals: embassy_nrf::Peripherals,
        spawner: &Spawner,
    ) -> Result<Self, SpawnError> {
        let pins = display::Pins {
            row1: peripherals.P0_21,
            row2: peripherals.P0_22,
            row3: peripherals.P0_15,
            row4: peripherals.P0_24,
            row5: peripherals.P0_19,
            col1: peripherals.P0_28,
            col2: peripherals.P0_11,
            col3: peripherals.P0_31,
            col4: peripherals.P1_05,
            col5: peripherals.P0_30,
        };

        Ok(Self {
            display: Display::new(pins, &spawner)?,
            button_a: Button::new(peripherals.P0_14),
            button_b: Button::new(peripherals.P0_23),
            uart: unsafe {
                let mut config = uarte::Config::default();
                config.baudrate = Baudrate::BAUD115200;
                config.parity = Parity::EXCLUDED;
                Uarte::new(
                    peripherals.UARTE0,
                    interrupt::take!(UARTE0_UART0),
                    peripherals.P1_08,
                    peripherals.P0_06,
                    NoPin,
                    NoPin,
                    config,
                )
            },
        })
    }
}
