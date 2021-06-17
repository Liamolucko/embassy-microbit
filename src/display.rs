use core::ops::Deref;
use core::ops::DerefMut;

use defmt::Format;
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

const SCROLL_DELAY: Duration = Duration::from_millis(150);

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

#[derive(Clone, Debug, Format)]
pub struct Image(pub [[u8; 5]; 5]);

impl Deref for Image {
    type Target = [[u8; 5]; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Image {
    pub const BLANK: Self = Image([[0; 5]; 5]);

    fn shift_left(&mut self, n: usize) {
        for row in self.0.iter_mut() {
            for i in 0..5 {
                row[i] = row.get(i + n).cloned().unwrap_or(0);
            }
        }
    }

    fn column_non_blank(&self, i: usize) -> bool {
        for row in self.0.iter() {
            if row[i] != 0 {
                return true;
            }
        }
        false
    }
}

impl From<char> for Image {
    fn from(char: char) -> Self {
        Image(match char {
            ' ' => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            '!' => [
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            '"' => [
                [0, 255, 0, 255, 0],
                [0, 255, 0, 255, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            '#' => [
                [0, 255, 0, 255, 0],
                [255, 255, 255, 255, 255],
                [0, 255, 0, 255, 0],
                [255, 255, 255, 255, 255],
                [0, 255, 0, 255, 0],
            ],
            '$' => [
                [0, 255, 255, 255, 0],
                [255, 255, 0, 0, 255],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 255, 255],
                [0, 255, 255, 255, 0],
            ],
            '%' => [
                [255, 255, 0, 0, 255],
                [255, 0, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 255],
                [255, 0, 0, 255, 255],
            ],
            '&' => [
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 255],
            ],
            '\'' => [
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            '(' => [
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 255, 0, 0],
            ],
            ')' => [
                [0, 255, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            '*' => [
                [0, 0, 0, 0, 0],
                [0, 255, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 255, 0],
                [0, 0, 0, 0, 0],
            ],
            '+' => [
                [0, 0, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 255, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            ',' => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            '-' => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            '.' => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            '/' => [
                [0, 0, 0, 0, 255],
                [0, 0, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [255, 0, 0, 0, 0],
            ],
            '0' => [
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            '1' => [
                [0, 0, 255, 0, 0],
                [0, 255, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 255, 255, 0],
            ],
            '2' => [
                [255, 255, 255, 0, 0],
                [0, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 255, 0],
            ],
            '3' => [
                [255, 255, 255, 255, 0],
                [0, 0, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            '4' => [
                [0, 0, 255, 255, 0],
                [0, 255, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 255, 255],
                [0, 0, 0, 255, 0],
            ],
            '5' => [
                [255, 255, 255, 255, 255],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 255, 0],
                [0, 0, 0, 0, 255],
                [255, 255, 255, 255, 0],
            ],
            '6' => [
                [0, 0, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 255],
                [0, 255, 255, 255, 0],
            ],
            '7' => [
                [255, 255, 255, 255, 255],
                [0, 0, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [255, 0, 0, 0, 0],
            ],
            '8' => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 255],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 255],
                [0, 255, 255, 255, 0],
            ],
            '9' => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 255],
                [0, 255, 255, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            ':' => [
                [0, 0, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            ';' => [
                [0, 0, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            '<' => [
                [0, 0, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 0, 255, 0],
            ],
            '=' => [
                [0, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [0, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [0, 0, 0, 0, 0],
            ],
            '>' => [
                [0, 255, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            '@' => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 255],
                [255, 0, 255, 0, 255],
                [255, 0, 0, 255, 255],
                [0, 255, 255, 0, 0],
            ],
            'A' => [
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
            ],
            'B' => [
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
            ],
            'C' => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
            ],
            'D' => [
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
            ],
            'E' => [
                [255, 255, 255, 255, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 255, 0],
            ],
            'F' => [
                [255, 255, 255, 255, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
            ],
            'G' => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 255, 255],
                [255, 0, 0, 0, 255],
                [0, 255, 255, 255, 0],
            ],
            'H' => [
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
            ],
            'I' => [
                [255, 255, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [255, 255, 255, 0, 0],
            ],
            'J' => [
                [255, 255, 255, 255, 255],
                [0, 0, 0, 255, 0],
                [0, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            'K' => [
                [255, 0, 0, 255, 0],
                [255, 0, 255, 0, 0],
                [255, 255, 0, 0, 0],
                [255, 0, 255, 0, 0],
                [255, 0, 0, 255, 0],
            ],
            'L' => [
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 255, 0],
            ],
            'M' => [
                [255, 0, 0, 0, 255],
                [255, 255, 0, 255, 255],
                [255, 0, 255, 0, 255],
                [255, 0, 0, 0, 255],
                [255, 0, 0, 0, 255],
            ],
            'N' => [
                [255, 0, 0, 0, 255],
                [255, 255, 0, 0, 255],
                [255, 0, 255, 0, 255],
                [255, 0, 0, 255, 255],
                [255, 0, 0, 0, 255],
            ],
            'O' => [
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            'P' => [
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
            ],
            'Q' => [
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
                [0, 0, 255, 255, 0],
            ],
            'R' => [
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 0, 255],
            ],
            'S' => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 0],
                [0, 255, 255, 0, 0],
                [0, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
            ],
            'T' => [
                [255, 255, 255, 255, 255],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
            ],
            'U' => [
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            'V' => [
                [255, 0, 0, 0, 255],
                [255, 0, 0, 0, 255],
                [255, 0, 0, 0, 255],
                [0, 255, 0, 255, 0],
                [0, 0, 255, 0, 0],
            ],
            'W' => [
                [255, 0, 0, 0, 255],
                [255, 0, 0, 0, 255],
                [255, 0, 255, 0, 255],
                [255, 255, 0, 255, 255],
                [255, 0, 0, 0, 255],
            ],
            'X' => [
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
            ],
            'Y' => [
                [255, 0, 0, 0, 255],
                [0, 255, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 0, 0],
            ],
            'Z' => [
                [255, 255, 255, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 255, 0],
            ],
            '[' => [
                [0, 255, 255, 255, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 255, 255, 0],
            ],
            '\\' => [
                [255, 0, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 0, 255, 0],
                [0, 0, 0, 0, 255],
            ],
            ']' => [
                [0, 255, 255, 255, 0],
                [0, 0, 0, 255, 0],
                [0, 0, 0, 255, 0],
                [0, 0, 0, 255, 0],
                [0, 255, 255, 255, 0],
            ],
            '^' => [
                [0, 0, 255, 0, 0],
                [0, 255, 0, 255, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            '_' => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [255, 255, 255, 255, 255],
            ],
            '`' => [
                [0, 255, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            'a' => [
                [0, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 255, 255],
            ],
            'b' => [
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
            ],
            'c' => [
                [0, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
            ],
            'd' => [
                [0, 0, 0, 255, 0],
                [0, 0, 0, 255, 0],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 255, 0],
            ],
            'e' => [
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
            ],
            'f' => [
                [0, 0, 255, 255, 0],
                [0, 255, 0, 0, 0],
                [255, 255, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            'g' => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 255, 0],
                [0, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            'h' => [
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
            ],
            'i' => [
                [0, 255, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            'j' => [
                [0, 0, 0, 255, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 255, 0],
                [0, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            'k' => [
                [255, 0, 0, 0, 0],
                [255, 0, 255, 0, 0],
                [255, 255, 0, 0, 0],
                [255, 0, 255, 0, 0],
                [255, 0, 0, 255, 0],
            ],
            'l' => [
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 255, 255, 0],
            ],
            'm' => [
                [0, 0, 0, 0, 0],
                [255, 255, 0, 255, 255],
                [255, 0, 255, 0, 255],
                [255, 0, 0, 0, 255],
                [255, 0, 0, 0, 255],
            ],
            'n' => [
                [0, 0, 0, 0, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
            ],
            'o' => [
                [0, 0, 0, 0, 0],
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
            ],
            'p' => [
                [0, 0, 0, 0, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 255, 255, 0, 0],
                [255, 0, 0, 0, 0],
            ],
            'q' => [
                [0, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 255, 0],
                [0, 0, 0, 255, 0],
            ],
            'r' => [
                [0, 0, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
                [255, 0, 0, 0, 0],
            ],
            's' => [
                [0, 0, 0, 0, 0],
                [0, 0, 255, 255, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 255, 0, 0],
                [255, 255, 0, 0, 0],
            ],
            't' => [
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 255, 255, 0],
                [0, 255, 0, 0, 0],
                [0, 0, 255, 255, 255],
            ],
            'u' => [
                [0, 0, 0, 0, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 255, 255],
            ],
            'v' => [
                [0, 0, 0, 0, 0],
                [255, 0, 0, 0, 255],
                [255, 0, 0, 0, 255],
                [0, 255, 0, 255, 0],
                [0, 0, 255, 0, 0],
            ],
            'w' => [
                [0, 0, 0, 0, 0],
                [255, 0, 0, 0, 255],
                [255, 0, 0, 0, 255],
                [255, 0, 255, 0, 255],
                [255, 255, 0, 255, 255],
            ],
            'x' => [
                [0, 0, 0, 0, 0],
                [255, 0, 0, 255, 0],
                [0, 255, 255, 0, 0],
                [0, 255, 255, 0, 0],
                [255, 0, 0, 255, 0],
            ],
            'y' => [
                [0, 0, 0, 0, 0],
                [255, 0, 0, 0, 255],
                [0, 255, 0, 255, 0],
                [0, 0, 255, 0, 0],
                [255, 255, 0, 0, 0],
            ],
            'z' => [
                [0, 0, 0, 0, 0],
                [255, 255, 255, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [255, 255, 255, 255, 0],
            ],
            '{' => [
                [0, 0, 255, 255, 0],
                [0, 0, 255, 0, 0],
                [0, 255, 255, 0, 0],
                [0, 0, 255, 0, 0],
                [0, 0, 255, 255, 0],
            ],
            '|' => [
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
            ],
            '}' => [
                [255, 255, 0, 0, 0],
                [0, 255, 0, 0, 0],
                [0, 255, 255, 0, 0],
                [0, 255, 0, 0, 0],
                [255, 255, 0, 0, 0],
            ],
            '~' => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 255, 255, 0, 0],
                [0, 0, 0, 255, 255],
                [0, 0, 0, 0, 0],
            ],

            // Unsupported characters become ?
            _ => [
                [0, 255, 255, 255, 0],
                [255, 0, 0, 0, 255],
                [0, 0, 255, 255, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 255, 0, 0],
            ],
        })
    }
}

// TODO: not sure if this is safe with interrupts and such
// Because `Display` can only be created with ownership of the correct pins, two `Display`s treading on each other's feet shouldn't be a concern.
static mut IMAGE: Option<Image> = None;

#[task]
/// `IMAGE` is set to `None` when the `Display` is dropped, which will cause this to return.
/// There isn't currently another way to cancel a task.
async fn render(
    mut rows: [gpio::Output<'static, AnyPin>; 5],
    mut cols: [gpio::Output<'static, AnyPin>; 5],
) {
    // Clone the image so it doesn't change whilst rendering it.
    while let Some(image) = unsafe { IMAGE.clone() } {
        for (row_pin, row) in rows.iter_mut().zip(image.0) {
            // How long we've already waited for
            let mut time_waited = Duration::from_secs(0);

            // These are infallible, `embedded-hal` just has them return errors in case there's a board out there with fallible pins.
            row_pin.set_high().unwrap();

            // Turn the whole row on, except the ones with brightness 0.
            for (col_pin, brightness) in cols.iter_mut().zip(row) {
                if brightness > 0 {
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
                // TODO: Embassy doesn't have it's RTC clocked high enough for all 255 brightness levels to actually be different;
                // either use a separate hardware timer or restrict the number of brightness levels.
                let delay =
                    (Duration::from_secs(1) * next_dimmest as u32) / (60 * 5 * 255) - time_waited;
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

pub struct Display(());

impl Display {
    /// Spawns a task to drive the display and returns a handle to set the display's image.
    pub fn new(pins: Pins, spawner: &Spawner) -> Result<Self, SpawnError> {
        unsafe {
            IMAGE = Some(Image::BLANK);
        }

        // TODO: figure out a way to implement a `take` method which gives the pins back
        // I think it's impossible right now (safely) because there's no `gpio::Output::take`.
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

        Ok(Self(()))
    }

    pub fn show(&mut self, image: Image) {
        unsafe { IMAGE = Some(image) }
    }

    pub async fn scroll(&mut self, text: &str) {
        let mut image = Image::BLANK;

        self.show(image.clone());
        Timer::after(SCROLL_DELAY).await;

        for next_image in text.chars().map(Image::from) {
            // Perform 'kerning' by skipping a few blank columns on the left and right.
            // Don't skip all of them so that spaces still exist.
            let start_pos = if next_image.column_non_blank(0) { 0 } else { 1 };
            let end_pos = if next_image.column_non_blank(4) {
                5
            } else if next_image.column_non_blank(3) {
                4
            } else {
                3
            };

            for i in start_pos..end_pos {
                image.shift_left(1);
                for (row, next_row) in image.iter_mut().zip(next_image.iter()) {
                    row[4] = next_row[i];
                }

                self.show(image.clone());
                Timer::after(SCROLL_DELAY).await;
            }

            // Add a column of space before the next character
            image.shift_left(1);
            self.show(image.clone());
            Timer::after(SCROLL_DELAY).await;
        }

        for _ in 0..4 {
            // Wait for the last character to scroll away.
            image.shift_left(1);
            self.show(image.clone());
            Timer::after(SCROLL_DELAY).await;
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        // This will trigger the task to stop running.
        unsafe { IMAGE = None }
    }
}
