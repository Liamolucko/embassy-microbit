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
use embassy_nrf::Peripherals;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: Peripherals) {
    let mut display = embassy_microbit::display!(peripherals, &spawner);
    let mut button_a = embassy_microbit::button_a!(peripherals, &spawner);
    let mut button_b = embassy_microbit::button_b!(peripherals, &spawner);

    let mut char = b'a';

    loop {
        if button_a.was_pressed() {
            char = char.wrapping_sub(1);
        }
        if button_b.was_pressed() {
            char = char.wrapping_add(1);
        }

        display.show(Image::from(char::from(char)));

        Timer::after(Duration::from_millis(10)).await;
    }
}
