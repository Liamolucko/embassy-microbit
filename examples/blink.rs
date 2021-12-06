//! Blinks an LED connected to pad 0 on the edge connector.

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;


use embassy::executor::Spawner;
use embassy::time::Duration;
use embassy::time::Timer;
use embassy_nrf::Peripherals;
use embassy_nrf::gpio;
use embassy_nrf::gpio::Level;
use embassy_nrf::gpio::OutputDrive;
use embedded_hal::digital::v2::OutputPin;

#[embassy::main]
async fn main(_spawner: Spawner, peripherals: Peripherals) {
    let mut pin = gpio::Output::new(embassy_microbit::pin0!(peripherals), Level::Low, OutputDrive::Standard);

    loop {
        Timer::after(Duration::from_millis(500)).await;
        pin.set_high().unwrap();
        Timer::after(Duration::from_millis(500)).await;
        pin.set_low().unwrap();
    }
}
