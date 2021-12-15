#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use embassy::executor::Spawner;
use embassy::time::{Duration, Instant, Timer};
use embassy_microbit::display::Image;
use embassy_nrf::Peripherals;

const FRAME_RATE: u64 = 60;
const FRAME_PERIOD: Duration = Duration::from_micros(1_000_000 / FRAME_RATE);

#[embassy::main]
async fn main(_spawner: Spawner, peripherals: Peripherals) {
    let mut display = embassy_microbit::display!(peripherals);

    let start = Instant::now();
    let mut next_frame = start + FRAME_PERIOD;
    loop {
        let elapsed = start.elapsed().as_millis() as f64 / 75.0;
        let mut image = [[0; 5]; 5];

        let origin_x = libm::cos(elapsed / 10.0) * 2.0 + 2.0;
        let origin_y = libm::sin(elapsed / 10.0) * 2.0 + 2.0;

        for x in 0..5 {
            for y in 0..5 {
                image[y][x] = (libm::pow(
                    libm::sin(
                        elapsed
                            - libm::sqrt(
                                libm::pow(x as f64 - origin_x, 2.0)
                                    + libm::pow(y as f64 - origin_y, 2.0),
                            ),
                    ) + 1.0,
                    2.0,
                ) * 63.75) as u8;
            }
        }

        display.show(Image(image));

        Timer::at(next_frame).await;

        next_frame += FRAME_PERIOD;
    }
}
