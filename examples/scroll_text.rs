#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use embassy::executor::Spawner;
use embassy_nrf::Peripherals;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: Peripherals) {
    let mut display = embassy_microbit::display!(peripherals, &spawner);

    display.scroll("Hello, World!").await;
}
