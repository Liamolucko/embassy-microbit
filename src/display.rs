use embassy::executor::SpawnError;
use embassy::executor::Spawner;
use embassy::task;
use embassy::time::Duration;
use embassy::time::Timer;
use embassy_nrf::gpio;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::Level;
use embassy_nrf::gpio::OutputDrive;
use embassy_nrf::gpio::Pin;
use embassy_nrf::peripherals::P0_11;
use embassy_nrf::peripherals::P0_15;
use embassy_nrf::peripherals::P0_19;
use embassy_nrf::peripherals::P0_21;
use embassy_nrf::peripherals::P0_22;
use embassy_nrf::peripherals::P0_24;
use embassy_nrf::peripherals::P0_28;
use embassy_nrf::peripherals::P0_30;
use embassy_nrf::peripherals::P0_31;
use embassy_nrf::peripherals::P1_05;
use embedded_hal::digital::v2::OutputPin;

const PERIOD: Duration = Duration::from_micros(1000000 / 60 / 5);

pub struct Pins {
    pub row1: P0_21,
    pub row2: P0_22,
    pub row3: P0_15,
    pub row4: P0_24,
    pub row5: P0_19,
    pub col1: P0_28,
    pub col2: P0_11,
    pub col3: P0_31,
    pub col4: P1_05,
    pub col5: P0_30,
}

type Image = [[u8; 5]; 5];
// TODO: not sure if this is safe with interrupts and such
// Because `Display` can only be created with ownership of the correct pins, two `Display`s treading on each other's feet shouldn't be a concern.
static mut IMAGE: Option<Image> = None;

#[task]
/// `image` is set to `None` when the `Display` is dropped, which will cause this to return.
/// There isn't currently another way to cancel a task.
async fn render(
    mut rows: [gpio::Output<'static, AnyPin>; 5],
    mut cols: [gpio::Output<'static, AnyPin>; 5],
) {
    // Clone the image so it doesn't change whilst rendering it.
    while let Some(image) = unsafe { IMAGE.clone() } {
        for (row_pin, row) in rows.iter_mut().zip(image.iter()) {
            // How long we've already waited for
            let mut time_waited = Duration::from_secs(0);

            // These are infallible, `embedded-hal` just has them return errors in case there's a board out there with fallible pins.
            row_pin.set_high().unwrap();

            // Turn the whole row on, except the ones with brightness 0.
            for (col_pin, brightness) in cols.iter_mut().zip(row.iter()) {
                if *brightness > 0 {
                    // The column pins are active low.
                    col_pin.set_low().unwrap();
                } else {
                    col_pin.set_high().unwrap();
                }
            }

            let mut next_dimmest = 0;

            while next_dimmest < 255 {
                // This will find the lowest brighness which is greater than `next_dimmest`, or default to 255.
                next_dimmest = row.iter().cloned().fold(255, |acc, brightness| {
                    if brightness > next_dimmest && brightness < acc {
                        brightness
                    } else {
                        acc
                    }
                });

                // How much longer we have to wait until we have to turn off the next set of LEDs.
                let delay = (PERIOD * next_dimmest as u32) / 255 - time_waited;
                time_waited += delay;

                Timer::after(delay).await;

                // Turn off the correct LEDs
                for (col_pin, brightness) in cols.iter_mut().zip(row.iter()) {
                    if *brightness == next_dimmest {
                        col_pin.set_high().unwrap();
                    }
                }
            }

            row_pin.set_low().unwrap();
        }
    }
}

pub struct Display {}

impl Display {
    /// Spawns a task to drive the display and returns a handle to set the display's image.
    pub fn new(pins: Pins, spawner: &Spawner) -> Result<Self, SpawnError> {
        unsafe {
            IMAGE = Some([[0; 5]; 5]);
        }

        spawner.spawn(render(
            [
                gpio::Output::new(pins.row1.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row2.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row3.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row4.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row5.degrade(), Level::Low, OutputDrive::Standard),
            ],
            [
                gpio::Output::new(pins.col1.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col2.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col3.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col4.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col5.degrade(), Level::High, OutputDrive::Standard),
            ],
        ))?;

        Ok(Self {})
    }

    pub fn show(&self, image: Image) {
        unsafe { IMAGE = Some(image) }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        // This will trigger the task to stop running.
        unsafe { IMAGE = None }
    }
}
