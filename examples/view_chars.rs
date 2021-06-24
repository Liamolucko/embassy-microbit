#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

extern crate defmt_rtt;
extern crate panic_probe;

use embassy::executor::Spawner;
use embassy::time::Duration;
use embassy::time::Timer;
use embassy_microbit::display::Image;
use embassy_microbit::Peripherals;
use embassy_microbit::RawPeripherals;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: RawPeripherals) {
    let mut peripherals = Peripherals::new(peripherals, &spawner);
    let mut display = peripherals.display;

    let mut char = b'a';

    loop {
        if peripherals.button_a.was_pressed() {
            char = char.wrapping_sub(1);
        }
        if peripherals.button_b.was_pressed() {
            char = char.wrapping_add(1);
        }

        display.show(Image::from(char::from(char)));

        Timer::after(Duration::from_millis(10)).await;
    }
}
