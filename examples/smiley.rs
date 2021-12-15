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
    let button_a = embassy_microbit::button_a!(peripherals, &spawner);
    let button_b = embassy_microbit::button_b!(peripherals, &spawner);

    loop {
        let mut image = Image::BLANK;

        if button_a.is_pressed() {
            image[1][1] = 255;
        }

        if button_b.is_pressed() {
            image[1][3] = 255;
        }

        if button_a.is_pressed() && button_b.is_pressed() {
            image[3][0] = 255;
            image[3][4] = 255;
            image[4][1] = 255;
            image[4][2] = 255;
            image[4][3] = 255;
        }

        display.show(image);

        Timer::after(Duration::from_millis(10)).await;
    }
}
