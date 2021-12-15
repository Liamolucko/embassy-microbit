#![no_std]
#![feature(type_alias_impl_trait)]

pub mod button;
pub mod display;
pub mod pins;

pub use button::Button;
pub use display::Display;

#[cfg(not(v2))]
#[macro_export]
macro_rules! display {
    ($peripherals:ident) => {{
        use ::embassy_nrf::interrupt;

        let pins = $crate::display::Pins {
            row1: $peripherals.P0_13,
            row2: $peripherals.P0_14,
            row3: $peripherals.P0_15,
            col1: $peripherals.P0_04,
            col2: $peripherals.P0_05,
            col3: $peripherals.P0_06,
            col4: $peripherals.P0_07,
            col5: $peripherals.P0_08,
            col6: $peripherals.P0_09,
            col7: $peripherals.P0_10,
            col8: $peripherals.P0_11,
            col9: $peripherals.P0_12,
        };

        $crate::Display::new(pins, $peripherals.TIMER1, interrupt::take!(TIMER1))
    }};
}

#[cfg(v2)]
#[macro_export]
macro_rules! display {
    ($peripherals:ident) => {{
        use ::embassy_nrf::interrupt;

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

        $crate::Display::new(pins, $peripherals.TIMER1, interrupt::take!(TIMER1))
    }};
}

#[cfg(not(v2))]
#[macro_export]
macro_rules! button_a {
    ($peripherals:expr, $spawner:expr) => {
        $crate::Button::new_a($peripherals.P0_17, $spawner)
    };
}

#[cfg(v2)]
#[macro_export]
macro_rules! button_a {
    ($peripherals:expr, $spawner:expr) => {
        $crate::Button::new_a($peripherals.P0_14, $spawner)
    };
}

#[cfg(not(v2))]
#[macro_export]
macro_rules! button_b {
    ($peripherals:expr, $spawner:expr) => {
        $crate::Button::new_b($peripherals.P0_26, $spawner)
    };
}

#[cfg(v2)]
#[macro_export]
macro_rules! button_b {
    ($peripherals:expr, $spawner:expr) => {
        $crate::Button::new_b($peripherals.P0_23, $spawner)
    };
}

#[cfg(not(v2))]
#[macro_export]
macro_rules! serial {
    ($peripherals:ident) => {
        unsafe {
            use ::embassy_nrf::gpio::NoPin;
            use ::embassy_nrf::interrupt;
            use ::embassy_nrf::uart;
            use ::embassy_nrf::uart::Baudrate;
            use ::embassy_nrf::uart::Parity;
            use ::embassy_nrf::uart::Uart;

            let mut config = uart::Config::default();
            config.baudrate = Baudrate::BAUD115200;
            config.parity = Parity::EXCLUDED;
            Uart::new(
                $peripherals.UART0,
                interrupt::take!(UART0),
                $peripherals.P0_25,
                $peripherals.P0_24,
                NoPin,
                NoPin,
                config,
            )
        }
    };
}

#[cfg(v2)]
#[macro_export]
macro_rules! serial {
    ($peripherals:ident) => {
        unsafe {
            use ::embassy_nrf::gpio::NoPin;
            use ::embassy_nrf::interrupt;
            use ::embassy_nrf::uart;
            use ::embassy_nrf::uart::Baudrate;
            use ::embassy_nrf::uart::Parity;
            use ::embassy_nrf::uart::Uart;

            let mut config = uart::Config::default();
            config.baudrate = Baudrate::BAUD115200;
            config.parity = Parity::EXCLUDED;
            Uart::new(
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
