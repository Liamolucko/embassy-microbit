//! Type aliases for pins, and macros to get the exposed pins of the edge connector.

use embassy_nrf::peripherals::P0_01;
use embassy_nrf::peripherals::P0_02;
use embassy_nrf::peripherals::P0_03;
use embassy_nrf::peripherals::P0_04;
use embassy_nrf::peripherals::P0_06;
use embassy_nrf::peripherals::P0_09;
use embassy_nrf::peripherals::P0_10;
use embassy_nrf::peripherals::P0_11;
use embassy_nrf::peripherals::P0_12;
use embassy_nrf::peripherals::P0_13;
use embassy_nrf::peripherals::P0_14;
use embassy_nrf::peripherals::P0_15;
use embassy_nrf::peripherals::P0_17;
use embassy_nrf::peripherals::P0_19;
use embassy_nrf::peripherals::P0_21;
use embassy_nrf::peripherals::P0_22;
use embassy_nrf::peripherals::P0_23;
use embassy_nrf::peripherals::P0_24;
use embassy_nrf::peripherals::P0_26;
use embassy_nrf::peripherals::P0_28;
use embassy_nrf::peripherals::P0_30;
use embassy_nrf::peripherals::P0_31;
use embassy_nrf::peripherals::P1_00;
use embassy_nrf::peripherals::P1_02;
use embassy_nrf::peripherals::P1_05;
use embassy_nrf::peripherals::P1_08;

pub type Row1 = P0_21;
pub type Row2 = P0_22;
pub type Row3 = P0_15;
pub type Row4 = P0_24;
pub type Row5 = P0_19;
pub type Col1 = P0_28;
pub type Col2 = P0_11;
pub type Col3 = P0_31;
pub type Col4 = P1_05;
pub type Col5 = P0_30;

pub type BtnA = P0_14;
pub type BtnB = P0_23;

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

pin!(pin0, Pin0 = P0_02);
pin!(pin1, Pin1 = P0_03);
pin!(pin2, Pin2 = P0_04);
pin!(pin3, Pin3 = P0_31);
pin!(pin4, Pin4 = P0_28);
pin!(pin5, Pin5 = P0_14);
pin!(pin6, Pin6 = P1_05);
pin!(pin7, Pin7 = P0_11);
pin!(pin8, Pin8 = P0_10);
pin!(pin9, Pin9 = P0_09);
pin!(pin10, Pin10 = P0_30);
pin!(pin11, Pin11 = P0_23);
pin!(pin12, Pin12 = P0_12);
pin!(pin13, Pin13 = P0_17);
pin!(pin14, Pin14 = P0_01);
pin!(pin15, Pin15 = P0_13);
pin!(pin16, Pin16 = P1_02);
// There are no pins 17-18 because there are 3V pins where they would otherwise be.
pin!(pin19, Pin19 = P0_26);
pin!(pin20, Pin20 = P1_00);

pin!(uart_rx, UartRx = P1_08);
pin!(uart_tx, UartTx = P0_06);
