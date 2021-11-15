#![no_std]
#![feature(type_alias_impl_trait)]

pub mod button;
pub mod display;
pub mod pins;

pub use button::Button;
pub use display::Display;

#[macro_export]
macro_rules! display {
    ($peripherals:ident, $spawner:expr) => {{
        let pins = $crate::display::Pins {
            row1: $peripherals.P0_21,
            row2: $peripherals.P0_22,
            row3: $peripherals.P0_15,
            row4: $peripherals.P0_24,
            row5: $peripherals.P0_19,
            col1: $peripherals.P0_28,
            col2: $peripherals.P0_11,
            col3: $peripherals.P0_31,
            col4: $peripherals.P1_05,
            col5: $peripherals.P0_30,
        };

        $crate::Display::new(pins, $spawner)
    }};
}

#[macro_export]
macro_rules! button_a {
    ($peripherals:expr, $spawner:expr) => {
        $crate::Button::new_a($peripherals.P0_14, $spawner)
    };
}

#[macro_export]
macro_rules! button_b {
    ($peripherals:expr, $spawner:expr) => {
        $crate::Button::new_b($peripherals.P0_23, $spawner)
    };
}

#[macro_export]
macro_rules! serial {
    ($peripherals:ident) => {
        unsafe {
            use ::embassy_nrf::gpio::NoPin;
            use ::embassy_nrf::interrupt;
            use ::embassy_nrf::uarte;
            use ::embassy_nrf::uarte::Baudrate;
            use ::embassy_nrf::uarte::Parity;
            use ::embassy_nrf::uarte::Uarte;

            let mut config = uarte::Config::default();
            config.baudrate = Baudrate::BAUD115200;
            config.parity = Parity::EXCLUDED;
            Uarte::new(
                $peripherals.UARTE0,
                interrupt::take!(UARTE0_UART0),
                $peripherals.P1_08,
                $peripherals.P0_06,
                NoPin,
                NoPin,
                config,
            )
        }
    };
}
