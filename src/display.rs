use core::ops::Deref;
use core::ops::DerefMut;

use defmt::Format;
use embassy::interrupt::Interrupt;
use embassy::interrupt::InterruptExt;
use embassy::time::Duration;
use embassy::time::Timer;
use embassy::util::Forever;
use embassy_hal_common::peripheral::PeripheralMutex;
use embassy_hal_common::peripheral::PeripheralState;
use embassy_hal_common::peripheral::StateStorage;
use embassy_nrf::gpio;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::Level;
use embassy_nrf::gpio::OutputDrive;
use embassy_nrf::gpio::Pin;
use embassy_nrf::interrupt;
use embassy_nrf::peripherals::TIMER1;
use embassy_nrf::timer::Timer as HwTimer;
use embedded_hal::digital::v2::OutputPin;

use crate::pins::Col1;
use crate::pins::Col2;
use crate::pins::Col3;
use crate::pins::Col4;
use crate::pins::Col5;
#[cfg(not(v2))]
use crate::pins::Col6;
#[cfg(not(v2))]
use crate::pins::Col7;
#[cfg(not(v2))]
use crate::pins::Col8;
#[cfg(not(v2))]
use crate::pins::Col9;
use crate::pins::Row1;
use crate::pins::Row2;
use crate::pins::Row3;
#[cfg(v2)]
use crate::pins::Row4;
#[cfg(v2)]
use crate::pins::Row5;

const SCROLL_DELAY: Duration = Duration::from_millis(150);

const REFRESH_RATE: u32 = 60;
// The timer's frequency is 1MHz, so 1s is 1_000_000 ticks.
const TICKS_PER_ROW: u16 = (1_000_000 / (REFRESH_RATE * HW_ROWS as u32)) as u16;
// Base this on the smaller value to make sure it's a clean multiple.
const TICKS_PER_FRAME: u16 = TICKS_PER_ROW * HW_ROWS as u16;

#[cfg(not(v2))]
const HW_ROWS: usize = 3;
#[cfg(not(v2))]
const HW_COLS: usize = 9;

#[cfg(v2)]
const HW_ROWS: usize = 5;
#[cfg(v2)]
const HW_COLS: usize = 5;

pub struct Pins {
    pub row1: Row1,
    pub row2: Row2,
    pub row3: Row3,
    #[cfg(v2)]
    pub row4: Row4,
    #[cfg(v2)]
    pub row5: Row5,
    pub col1: Col1,
    pub col2: Col2,
    pub col3: Col3,
    pub col4: Col4,
    pub col5: Col5,
    #[cfg(not(v2))]
    pub col6: Col6,
    #[cfg(not(v2))]
    pub col7: Col7,
    #[cfg(not(v2))]
    pub col8: Col8,
    #[cfg(not(v2))]
    pub col9: Col9,
}

#[derive(Clone, Debug, Format, PartialEq, Eq)]
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

    /// Unpack a 'compressed' image, where each row is a u8 with each bit representing an LED being on or off.
    /// Used to reduce binary size taken by font.
    fn unpack(data: [u8; 5]) -> Self {
        fn unpack(row: u8) -> [u8; 5] {
            [
                if row & 0b10000 != 0 { 255 } else { 0 },
                if row & 0b01000 != 0 { 255 } else { 0 },
                if row & 0b00100 != 0 { 255 } else { 0 },
                if row & 0b00010 != 0 { 255 } else { 0 },
                if row & 0b00001 != 0 { 255 } else { 0 },
            ]
        }
        Self([
            unpack(data[0]),
            unpack(data[1]),
            unpack(data[2]),
            unpack(data[3]),
            unpack(data[4]),
        ])
    }

    fn hw_rows(&self) -> [[u8; HW_COLS]; HW_ROWS] {
        #[cfg(not(v2))]
        return [
            [
                self[0][0], self[0][2], self[0][4], self[3][4], self[3][3], self[3][2], self[3][1],
                self[3][0], self[2][1],
            ],
            [
                self[2][4], self[2][0], self[2][2], self[0][1], self[0][3], self[4][3], self[4][1],
                0, 0,
            ],
            [
                self[4][2], self[4][4], self[4][0], self[1][0], self[1][1], self[1][2], self[1][3],
                self[1][4], self[2][3],
            ],
        ];
        #[cfg(v2)]
        self.0
    }

    fn steps(&self) -> [[(u16, usize); HW_COLS]; HW_ROWS] {
        let hw_rows = self.hw_rows();

        let mut out = [[(0, 0); HW_COLS]; HW_ROWS];
        for (i, row) in out.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = (
                    (TICKS_PER_ROW as u32 * hw_rows[i][j] as u32 / 255) as u16,
                    j,
                )
            }

            row.sort_unstable_by_key(|&(time, _)| time);
        }
        out
    }
}

impl From<char> for Image {
    fn from(char: char) -> Self {
        Image::unpack(match char {
            ' ' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
            '!' => [0b01000, 0b01000, 0b01000, 0b00000, 0b01000],
            '"' => [0b01010, 0b01010, 0b00000, 0b00000, 0b00000],
            '#' => [0b01010, 0b11111, 0b01010, 0b11111, 0b01010],
            '$' => [0b01110, 0b11001, 0b01110, 0b10011, 0b01110],
            '%' => [0b11001, 0b10010, 0b00100, 0b01001, 0b10011],
            '&' => [0b01100, 0b10010, 0b01100, 0b10010, 0b01101],
            '\'' => [0b01000, 0b01000, 0b00000, 0b00000, 0b00000],
            '(' => [0b00100, 0b01000, 0b01000, 0b01000, 0b00100],
            ')' => [0b01000, 0b00100, 0b00100, 0b00100, 0b01000],
            '*' => [0b00000, 0b01010, 0b00100, 0b01010, 0b00000],
            '+' => [0b00000, 0b00100, 0b01110, 0b00100, 0b00000],
            ',' => [0b00000, 0b00000, 0b00000, 0b00100, 0b01000],
            '-' => [0b00000, 0b00000, 0b01110, 0b00000, 0b00000],
            '.' => [0b00000, 0b00000, 0b00000, 0b01000, 0b00000],
            '/' => [0b00001, 0b00010, 0b00100, 0b01000, 0b10000],
            '0' => [0b01100, 0b10010, 0b10010, 0b10010, 0b01100],
            '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b01110],
            '2' => [0b11100, 0b00010, 0b01100, 0b10000, 0b11110],
            '3' => [0b11110, 0b00010, 0b00100, 0b10010, 0b01100],
            '4' => [0b00110, 0b01010, 0b10010, 0b11111, 0b00010],
            '5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b11110],
            '6' => [0b00010, 0b00100, 0b01110, 0b10001, 0b01110],
            '7' => [0b11111, 0b00010, 0b00100, 0b01000, 0b10000],
            '8' => [0b01110, 0b10001, 0b01110, 0b10001, 0b01110],
            '9' => [0b01110, 0b10001, 0b01110, 0b00100, 0b01000],
            ':' => [0b00000, 0b01000, 0b00000, 0b01000, 0b00000],
            ';' => [0b00000, 0b00100, 0b00000, 0b00100, 0b01000],
            '<' => [0b00010, 0b00100, 0b01000, 0b00100, 0b00010],
            '=' => [0b00000, 0b01110, 0b00000, 0b01110, 0b00000],
            '>' => [0b01000, 0b00100, 0b00010, 0b00100, 0b01000],
            '@' => [0b01110, 0b10001, 0b10101, 0b10011, 0b01100],
            'A' => [0b01100, 0b10010, 0b11110, 0b10010, 0b10010],
            'B' => [0b11100, 0b10010, 0b11100, 0b10010, 0b11100],
            'C' => [0b01110, 0b10000, 0b10000, 0b10000, 0b01110],
            'D' => [0b11100, 0b10010, 0b10010, 0b10010, 0b11100],
            'E' => [0b11110, 0b10000, 0b11100, 0b10000, 0b11110],
            'F' => [0b11110, 0b10000, 0b11100, 0b10000, 0b10000],
            'G' => [0b01110, 0b10000, 0b10011, 0b10001, 0b01110],
            'H' => [0b10010, 0b10010, 0b11110, 0b10010, 0b10010],
            'I' => [0b11100, 0b01000, 0b01000, 0b01000, 0b11100],
            'J' => [0b11111, 0b00010, 0b00010, 0b10010, 0b01100],
            'K' => [0b10010, 0b10100, 0b11000, 0b10100, 0b10010],
            'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b11110],
            'M' => [0b10001, 0b11011, 0b10101, 0b10001, 0b10001],
            'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001],
            'O' => [0b01100, 0b10010, 0b10010, 0b10010, 0b01100],
            'P' => [0b11100, 0b10010, 0b11100, 0b10000, 0b10000],
            'Q' => [0b01100, 0b10010, 0b10010, 0b01100, 0b00110],
            'R' => [0b11100, 0b10010, 0b11100, 0b10010, 0b10001],
            'S' => [0b01110, 0b10000, 0b01100, 0b00010, 0b11100],
            'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100],
            'U' => [0b10010, 0b10010, 0b10010, 0b10010, 0b01100],
            'V' => [0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
            'W' => [0b10001, 0b10001, 0b10101, 0b11011, 0b10001],
            'X' => [0b10010, 0b10010, 0b01100, 0b10010, 0b10010],
            'Y' => [0b10001, 0b01010, 0b00100, 0b00100, 0b00100],
            'Z' => [0b11110, 0b00100, 0b01000, 0b10000, 0b11110],
            '[' => [0b01110, 0b01000, 0b01000, 0b01000, 0b01110],
            '\\' => [0b10000, 0b01000, 0b00100, 0b00010, 0b00001],
            ']' => [0b01110, 0b00010, 0b00010, 0b00010, 0b01110],
            '^' => [0b00100, 0b01010, 0b00000, 0b00000, 0b00000],
            '_' => [0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
            '`' => [0b01000, 0b00100, 0b00000, 0b00000, 0b00000],
            'a' => [0b00000, 0b01110, 0b10010, 0b10010, 0b01111],
            'b' => [0b10000, 0b10000, 0b11100, 0b10010, 0b11100],
            'c' => [0b00000, 0b01110, 0b10000, 0b10000, 0b01110],
            'd' => [0b00010, 0b00010, 0b01110, 0b10010, 0b01110],
            'e' => [0b01100, 0b10010, 0b11100, 0b10000, 0b01110],
            'f' => [0b00110, 0b01000, 0b11100, 0b01000, 0b01000],
            'g' => [0b01110, 0b10010, 0b01110, 0b00010, 0b01100],
            'h' => [0b10000, 0b10000, 0b11100, 0b10010, 0b10010],
            'i' => [0b01000, 0b00000, 0b01000, 0b01000, 0b01000],
            'j' => [0b00010, 0b00000, 0b00010, 0b00010, 0b01100],
            'k' => [0b10000, 0b10100, 0b11000, 0b10100, 0b10010],
            'l' => [0b01000, 0b01000, 0b01000, 0b01000, 0b00110],
            'm' => [0b00000, 0b11011, 0b10101, 0b10001, 0b10001],
            'n' => [0b00000, 0b11100, 0b10010, 0b10010, 0b10010],
            'o' => [0b00000, 0b01100, 0b10010, 0b10010, 0b01100],
            'p' => [0b00000, 0b11100, 0b10010, 0b11100, 0b10000],
            'q' => [0b00000, 0b01110, 0b10010, 0b01110, 0b00010],
            'r' => [0b00000, 0b01110, 0b10000, 0b10000, 0b10000],
            's' => [0b00000, 0b00110, 0b01000, 0b00100, 0b11000],
            't' => [0b01000, 0b01000, 0b01110, 0b01000, 0b00111],
            'u' => [0b00000, 0b10010, 0b10010, 0b10010, 0b01111],
            'v' => [0b00000, 0b10001, 0b10001, 0b01010, 0b00100],
            'w' => [0b00000, 0b10001, 0b10001, 0b10101, 0b11011],
            'x' => [0b00000, 0b10010, 0b01100, 0b01100, 0b10010],
            'y' => [0b00000, 0b10001, 0b01010, 0b00100, 0b11000],
            'z' => [0b00000, 0b11110, 0b00100, 0b01000, 0b11110],
            '{' => [0b00110, 0b00100, 0b01100, 0b00100, 0b00110],
            '|' => [0b01000, 0b01000, 0b01000, 0b01000, 0b01000],
            '}' => [0b11000, 0b01000, 0b01100, 0b01000, 0b11000],
            '~' => [0b00000, 0b00000, 0b01100, 0b00011, 0b00000],

            // Unsupported characters become ?
            _ => [0b01110, 0b10001, 0b00110, 0b00000, 0b00100],
        })
    }
}

struct DisplayState {
    next_steps: [[(u16, usize); HW_COLS]; HW_ROWS],
    steps: [[(u16, usize); HW_COLS]; HW_ROWS],

    row: usize,
    /// The current 'step' of the current row; the nth step is when the nth dimmest column is turned off.
    step: usize,

    row_pins: [gpio::Output<'static, AnyPin>; HW_ROWS],
    col_pins: [gpio::Output<'static, AnyPin>; HW_COLS],

    timer: HwTimer<'static, TIMER1, u16>,
}

impl DisplayState {
    fn time(&mut self) -> u16 {
        // Don't use a modulus for this so that things don't get messed up
        // if the timer hits the next row midway through the interrupt.
        self.timer.cc(2).capture() - self.row as u16 * TICKS_PER_ROW
    }
}

impl PeripheralState for DisplayState {
    type Interrupt = interrupt::TIMER1;

    // This is written to do everything based on the timer's current value, rather then the numbre of times it's triggered.
    fn on_interrupt(&mut self) {
        // Clear the events so this interrupt doesn't get repeatedly fired.
        // TODO: Make a proper binding for this.
        unsafe {
            let reg = &*embassy_nrf::pac::TIMER1::ptr();
            reg.events_compare[0].reset();
            reg.events_compare[1].reset();
        }

        let row = self.timer.cc(2).capture() / TICKS_PER_ROW;
        let row = row as usize;

        if row != self.row {
            // The row has changed; start rendering a new one.

            // Turn off any remaining columns.
            for pin in &mut self.col_pins {
                pin.set_high().unwrap();
            }

            // Disable the previous row's pin.
            self.row_pins[self.row].set_low().unwrap();

            self.row = row;
            self.step = 0;

            if self.row == 0 {
                // Update the image we're displaying at the start of each frame.
                self.steps = self.next_steps;
            }

            self.row_pins[self.row].set_high().unwrap();

            // Turn on all the pins which aren't supposed to be completely off.
            for (time, col) in self.steps[self.row] {
                if time > 0 {
                    // The column pins are active low.
                    self.col_pins[col].set_low().unwrap();
                } else {
                    // We don't need to step through any columns which weren't on to begin with.
                    self.step += 1;
                }
            }
        }

        let steps = self.steps[self.row];

        // Turn off all of the columns whose times have passed.
        while self.step < HW_COLS && steps[self.step].0 <= self.time() {
            let (_, col) = steps[self.step];

            self.col_pins[col].set_high().unwrap();

            self.step += 1;
        }

        let time = self.steps[self.row]
            .get(self.step)
            // Default to `TICKS_PER_ROW` if there are none left, since we then just want to wait until we reach the next row.
            .map_or(TICKS_PER_ROW, |&(time, _)| time);

        self.timer
            .cc(0)
            .write(self.row as u16 * TICKS_PER_ROW + time);

        // Start the timer if it isn't already running.
        self.timer.start();

        if self.timer.cc(2).capture() >= self.timer.cc(0).read()
            && !unsafe { interrupt::TIMER1::steal() }.is_pending()
        {
            // It ticked past between the loop and here, so just trigger this handler again.
            self.on_interrupt();
        }
    }
}

static STATE: Forever<StateStorage<DisplayState>> = Forever::new();

pub struct Display {
    mutex: PeripheralMutex<'static, DisplayState>,
}

impl Display {
    /// Spawns a task to drive the display and returns a handle to set the display's image.
    pub fn new(pins: Pins, timer: TIMER1, irq: interrupt::TIMER1) -> Self {
        let mut state = DisplayState {
            timer: HwTimer::new(timer),

            next_steps: Image::BLANK.steps(),
            steps: Image::BLANK.steps(),
            // Initialize the state such that it'll immediately reset itself.
            row: HW_ROWS - 1,
            step: HW_COLS,

            #[cfg(v2)]
            row_pins: [
                gpio::Output::new(pins.row1.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row2.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row3.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row4.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row5.degrade(), Level::Low, OutputDrive::Standard),
            ],
            #[cfg(not(v2))]
            row_pins: [
                gpio::Output::new(pins.row1.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row2.degrade(), Level::Low, OutputDrive::Standard),
                gpio::Output::new(pins.row3.degrade(), Level::Low, OutputDrive::Standard),
            ],

            #[cfg(v2)]
            col_pins: [
                gpio::Output::new(pins.col1.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col2.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col3.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col4.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col5.degrade(), Level::High, OutputDrive::Standard),
            ],
            #[cfg(not(v2))]
            col_pins: [
                gpio::Output::new(pins.col1.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col2.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col3.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col4.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col5.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col6.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col7.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col8.degrade(), Level::High, OutputDrive::Standard),
                gpio::Output::new(pins.col9.degrade(), Level::High, OutputDrive::Standard),
            ],
        };

        // Make the timer reset itself at the end of each frame.
        state.timer.cc(1).write(TICKS_PER_FRAME);
        state.timer.cc(1).short_compare_clear();
        // Enable an interrupt when CC 0 or 1's value is reached.
        // TODO: Make a proper binding for this.
        unsafe {
            let reg = &*embassy_nrf::pac::TIMER1::ptr();
            reg.intenset
                .write(|w| w.compare0().set_bit().compare1().set_bit());
        }

        irq.pend();

        let mutex = PeripheralMutex::new(irq, STATE.put(StateStorage::new()), || state);

        Self { mutex }
    }

    pub fn show(&mut self, image: Image) {
        self.mutex.with(|state| state.next_steps = image.steps());
    }

    pub async fn scroll(&mut self, text: &str) {
        let mut image = Image::BLANK;

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

        // Let the last character scroll away.
        for _ in 0..4 {
            image.shift_left(1);
            self.show(image.clone());
            Timer::after(SCROLL_DELAY).await;
        }
    }
}
