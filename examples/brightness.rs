#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

use core::mem;

use defmt_rtt as _;
use panic_probe as _;

use embassy::executor::Spawner;
use embassy_microbit::display;
use embassy_microbit::display::Display;
use embassy_nrf::interrupt;
use embassy_nrf::Peripherals;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: Peripherals) {
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

    let display = Display::new(pins, &spawner).unwrap();

    let image = [
        [10, 20, 30, 40, 50],
        [60, 70, 80, 90, 100],
        [110, 120, 130, 140, 150],
        [160, 170, 180, 190, 200],
        [210, 220, 230, 240, 250],
    ];

    display.show(image);

    // Dropping the `Display` will cancel rendering.
    mem::forget(display);
}
