#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use embassy::executor::Spawner;
use embassy::traits::uart::Read;
use embassy::traits::uart::Write;
use embassy_nrf::Peripherals;

#[embassy::main]
async fn main(_spawner: Spawner, peripherals: Peripherals) {
    let mut uart = embassy_microbit::serial!(peripherals);

    let mut buf = [0];
    loop {
        uart.read(&mut buf).await.unwrap();
        uart.write(&buf).await.unwrap();
    }
}
