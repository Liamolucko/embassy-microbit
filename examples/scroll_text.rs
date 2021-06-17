#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

use defmt_rtt as _;
use panic_probe as _;

use embassy::executor::Spawner;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: embassy_nrf::Peripherals) {
    let peripherals = embassy_microbit::Peripherals::new(peripherals, &spawner).unwrap();

    let mut display = peripherals.display;

    display.scroll("Hello, World!").await;
}
