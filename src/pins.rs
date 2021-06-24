//! Type aliases for pins.

use embassy_nrf::peripherals::P0_11;
use embassy_nrf::peripherals::P0_14;
use embassy_nrf::peripherals::P0_15;
use embassy_nrf::peripherals::P0_19;
use embassy_nrf::peripherals::P0_21;
use embassy_nrf::peripherals::P0_22;
use embassy_nrf::peripherals::P0_23;
use embassy_nrf::peripherals::P0_24;
use embassy_nrf::peripherals::P0_28;
use embassy_nrf::peripherals::P0_30;
use embassy_nrf::peripherals::P0_31;
use embassy_nrf::peripherals::P1_05;

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
