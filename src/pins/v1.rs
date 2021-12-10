//! Type aliases for pins, and macros to get the exposed pins of the edge connector.

use embassy_nrf::peripherals::P0_00;
use embassy_nrf::peripherals::P0_01;
use embassy_nrf::peripherals::P0_02;
use embassy_nrf::peripherals::P0_03;
use embassy_nrf::peripherals::P0_04;
use embassy_nrf::peripherals::P0_05;
use embassy_nrf::peripherals::P0_06;
use embassy_nrf::peripherals::P0_07;
use embassy_nrf::peripherals::P0_08;
use embassy_nrf::peripherals::P0_09;
use embassy_nrf::peripherals::P0_10;
use embassy_nrf::peripherals::P0_11;
use embassy_nrf::peripherals::P0_12;
use embassy_nrf::peripherals::P0_13;
use embassy_nrf::peripherals::P0_14;
use embassy_nrf::peripherals::P0_15;
use embassy_nrf::peripherals::P0_16;
use embassy_nrf::peripherals::P0_17;
use embassy_nrf::peripherals::P0_18;
use embassy_nrf::peripherals::P0_20;
use embassy_nrf::peripherals::P0_21;
use embassy_nrf::peripherals::P0_22;
use embassy_nrf::peripherals::P0_23;
use embassy_nrf::peripherals::P0_24;
use embassy_nrf::peripherals::P0_25;
use embassy_nrf::peripherals::P0_26;
use embassy_nrf::peripherals::P0_30;

pub type Row1 = P0_13;
pub type Row2 = P0_14;
pub type Row3 = P0_15;
pub type Col1 = P0_04;
pub type Col2 = P0_05;
pub type Col3 = P0_06;
pub type Col4 = P0_07;
pub type Col5 = P0_08;
pub type Col6 = P0_09;
pub type Col7 = P0_10;
pub type Col8 = P0_11;
pub type Col9 = P0_12;

pub type BtnA = P0_17;
pub type BtnB = P0_26;

macro_rules! pin {
    ($lower_name:ident, $name:ident = $periph:ident) => {
        pub type $name = $periph;
        #[macro_export]
        macro_rules! $lower_name {
            ($peripherals:ident) => {
                $peripherals.$periph
            };
        }
    };
}

pin!(pin0, Pin0 = P0_03);
pin!(pin1, Pin1 = P0_02);
pin!(pin2, Pin2 = P0_01);
pin!(pin3, Pin3 = P0_04);
pin!(pin4, Pin4 = P0_05);
pin!(pin5, Pin5 = P0_17);
pin!(pin6, Pin6 = P0_12);
pin!(pin7, Pin7 = P0_11);
pin!(pin8, Pin8 = P0_18);
pin!(pin9, Pin9 = P0_10);
pin!(pin10, Pin10 = P0_06);
pin!(pin11, Pin11 = P0_26);
pin!(pin12, Pin12 = P0_20);
pin!(pin13, Pin13 = P0_23);
pin!(pin14, Pin14 = P0_22);
pin!(pin15, Pin15 = P0_21);
pin!(pin16, Pin16 = P0_16);
// There are no pins 17-18 because there are 3V pins where they would otherwise be.
pin!(pin19, Pin19 = P0_00);
pin!(pin20, Pin20 = P0_30);

pin!(uart_rx, UartRx = P0_25);
pin!(uart_tx, UartTx = P0_24);
