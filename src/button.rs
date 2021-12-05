use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use embassy::blocking_mutex::CriticalSectionMutex;
use embassy::executor::Spawner;
use embassy::task;
use embassy::time::Duration;
use embassy::time::Timer;
use embassy_nrf::gpio;
use embassy_nrf::gpio::Pin;
use embassy_nrf::gpio::Pull;
use embedded_hal::digital::v2::InputPin;
use once_cell::unsync::OnceCell;

use crate::pins::BtnA;
use crate::pins::BtnB;

static A_INPUT: CriticalSectionMutex<OnceCell<gpio::Input<'static, BtnA>>> =
    CriticalSectionMutex::new(OnceCell::new());
static B_INPUT: CriticalSectionMutex<OnceCell<gpio::Input<'static, BtnB>>> =
    CriticalSectionMutex::new(OnceCell::new());

static A_PRESS_COUNT: AtomicU32 = AtomicU32::new(0);
static B_PRESS_COUNT: AtomicU32 = AtomicU32::new(0);

#[task]
async fn poll_buttons() {
    // This algorithm was taken from the official micro:bit runtime (https://github.com/lancaster-university/codal-core/blob/master/source/drivers/Button.cpp)
    const MIN_SIGMA: u8 = 0;
    const MAX_SIGMA: u8 = 12;

    const SIGMA_HIGH_THRESHOLD: u8 = 8;
    const SIGMA_LOW_THRESHOLD: u8 = 2;

    let mut a_sigma: u8 = 0;
    let mut a_pressed = false;

    let mut b_sigma: u8 = 0;
    let mut b_pressed = false;

    loop {
        critical_section::with(|cs| {
            if let Some(pin) = A_INPUT.borrow(cs).get() {
                if pin.is_low().unwrap() {
                    if a_sigma < MAX_SIGMA {
                        a_sigma += 1;
                    }
                } else if a_sigma > MIN_SIGMA {
                    a_sigma -= 1;
                }

                if a_sigma > SIGMA_HIGH_THRESHOLD && !a_pressed {
                    A_PRESS_COUNT.fetch_add(1, Ordering::Relaxed);
                    a_pressed = true;
                } else if a_sigma < SIGMA_LOW_THRESHOLD && a_pressed {
                    a_pressed = false;
                }
            }

            if let Some(pin) = B_INPUT.borrow(cs).get() {
                if pin.is_low().unwrap() {
                    if b_sigma < MAX_SIGMA {
                        b_sigma += 1;
                    }
                } else if b_sigma > MIN_SIGMA {
                    b_sigma -= 1;
                }

                if b_sigma > SIGMA_HIGH_THRESHOLD && !b_pressed {
                    B_PRESS_COUNT.fetch_add(1, Ordering::Relaxed);
                    b_pressed = true;
                } else if b_sigma < SIGMA_LOW_THRESHOLD && b_pressed {
                    b_pressed = false;
                }
            }
        });

        Timer::after(Duration::from_millis(1)).await;
    }
}

pub struct Button<T: Pin> {
    pin: &'static CriticalSectionMutex<OnceCell<gpio::Input<'static, T>>>,
    press_count: &'static AtomicU32,
    /// The value of `press_count` last time `was_pressed` was called.
    last_press_count: u32,
}

impl Button<BtnA> {
    pub fn new_a(pin: BtnA, spawner: &Spawner) -> Button<BtnA> {
        critical_section::with(|cs| {
            A_INPUT
                .borrow(cs)
                .set(gpio::Input::new(pin, Pull::None))
                .map_err(|_| ()) // gpio::Input doesn't impl `Debug`
                .expect("Button A input already set")
        });

        // Ignore errors, because it's perfectly fine for the other button to have already spawned the polling task
        let _ = spawner.spawn(poll_buttons());

        Self {
            pin: &A_INPUT,
            press_count: &A_PRESS_COUNT,
            last_press_count: 0,
        }
    }
}

impl Button<BtnB> {
    pub fn new_b(pin: BtnB, spawner: &Spawner) -> Button<BtnB> {
        critical_section::with(|cs| {
            B_INPUT
                .borrow(cs)
                .set(gpio::Input::new(pin, Pull::None))
                .map_err(|_| ()) // gpio::Input doesn't impl `Debug`
                .expect("Button B input already set")
        });

        // Ignore errors, because it's perfectly fine for the other button to have already spawned the polling task
        let _ = spawner.spawn(poll_buttons());

        Self {
            pin: &B_INPUT,
            press_count: &B_PRESS_COUNT,
            last_press_count: 0,
        }
    }
}

impl<T: Pin> Button<T> {
    fn with_pin<R>(&self, f: impl FnOnce(&gpio::Input<'static, T>) -> R) -> R {
        critical_section::with(|cs| f(self.pin.borrow(cs).get().unwrap()))
    }

    pub fn is_pressed(&self) -> bool {
        self.with_pin(|pin| pin.is_low().unwrap())
    }

    pub fn was_pressed(&mut self) -> bool {
        let press_count = self.press_count.load(Ordering::Relaxed);
        let was_pressed = press_count > self.last_press_count;
        self.last_press_count = press_count;
        was_pressed
    }
}
