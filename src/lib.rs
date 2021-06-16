#![no_std]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

use display::Display;
use embassy::executor::SpawnError;
use embassy::executor::Spawner;

pub mod display;

pub struct Peripherals {
    pub display: Display,
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
        })
    }
}
