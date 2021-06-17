#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

use defmt_rtt as _;
use panic_probe as _;

use embassy::executor::Spawner;
use embassy::time::Duration;
use embassy::time::Timer;
use embassy_microbit::display::Image;

#[embassy::main]
async fn main(spawner: Spawner, peripherals: embassy_nrf::Peripherals) {
    let peripherals = embassy_microbit::Peripherals::new(peripherals, &spawner).unwrap();

    let mut display = peripherals.display;

    loop {
        let mut image = Image::BLANK;

        if peripherals.button_a.is_pressed() {
            image[1][1] = 255;
        }

        if peripherals.button_b.is_pressed() {
            image[1][3] = 255;
        }

        if peripherals.button_a.is_pressed() && peripherals.button_b.is_pressed() {
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
