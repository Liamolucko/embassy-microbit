#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

use defmt_rtt as _;
use panic_probe as _;

use core::mem;

use embassy::executor::Spawner;
use embassy_microbit::display::Image;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: embassy_nrf::Peripherals) {
    let peripherals = embassy_microbit::Peripherals::new(peripherals, &spawner).unwrap();

    let mut display = peripherals.display;

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
