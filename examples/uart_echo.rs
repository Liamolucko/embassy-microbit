#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

extern crate defmt_rtt;
extern crate panic_probe;

use embassy::executor::Spawner;
use embassy::traits::uart::Read;
use embassy::traits::uart::Write;
use embassy_microbit::Peripherals;
use embassy_microbit::RawPeripherals;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: RawPeripherals) {
    let mut peripherals = Peripherals::new(peripherals, &spawner).unwrap();

    let mut buf = [0];
    loop {
        peripherals.uart.read(&mut buf).await.unwrap();
        peripherals.uart.write(&buf).await.unwrap();
    }
}
