#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use embassy::executor::Spawner;
use embassy::time::Duration;
use embassy::time::Timer;
use embassy_microbit::display::Image;
use embassy_nrf::Peripherals;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: Peripherals) {
    let mut display = embassy_microbit::display!(peripherals);
    let mut button_a = embassy_microbit::button_a!(peripherals, &spawner);
    let mut button_b = embassy_microbit::button_b!(peripherals, &spawner);

    let mut pixel = 0;

    loop {
        if button_a.was_pressed() {
            if pixel == 0 {
                pixel = 24;
            } else {
                pixel -= 1;
            }
        }

        if button_b.was_pressed() {
            pixel += 1;
            if pixel == 25 {
                pixel = 0;
            }
        }

        let mut image = Image::BLANK;

        image[pixel / 5][pixel % 5] = 255;

        display.show(image);

        Timer::after(Duration::from_millis(10)).await;
    }
}
