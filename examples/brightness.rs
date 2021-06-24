#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

extern crate defmt_rtt;
extern crate panic_probe;

use core::mem;

use embassy::executor::Spawner;
use embassy_microbit::display::Image;
use embassy_nrf::Peripherals;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: Peripherals) {
    let mut display = embassy_microbit::display!(peripherals, &spawner);

    let image = Image([
        [10, 20, 30, 40, 50],
        [60, 70, 80, 90, 100],
        [110, 120, 130, 140, 150],
        [160, 170, 180, 190, 200],
        [210, 220, 230, 240, 250],
    ]);

    display.show(image);

    // Dropping the `Display` will cancel rendering.
    mem::forget(display);
}
