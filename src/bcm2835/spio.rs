extern crate libc;

use self::libc::{uint16_t, uint8_t};

pub enum BitOrder {
    LSBFirst = 0,
    MSBFirst = 1,
}

pub enum Mode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
    Mode3 = 3,
}

pub enum ClockDivider {
    Divider1 = 1,
    Divider2 = 2,
    Divider4 = 4,
    Divider8 = 8,
    Divider16 = 16,
    Divider32 = 32,
    Divider64 = 64,
    Divider128 = 128,
    Divider256 = 256,
    Divider512 = 512,
    Divider1024 = 1024,
    Divider2048 = 2048,
    Divider4096 = 4096,
    Divider8192 = 8192,
    Divider16384 = 16384,
    Divider32768 = 32768,
    Divider65536 = 65536,
}

pub enum ChipSelect {
    CS0 = 0,
    CS1 = 1,
    CS2 = 2,
    CSNone = 3,
}


pub fn begin() -> Result<(), (&'static str, i32)> {
    let result = unsafe {super::bcm2835_spi_begin()};
    if result == 1 {
        return Ok(());
    } else if result == 0 {
        return Err(("failed to initialize spi! Are you root?", 0));
    }
    Err(("Initializing spi failed with unknown code", result))
}

pub fn end() {
    unsafe { super::bcm2835_spi_end() };
}

pub fn transfer(value: u8) -> u8 {
    unsafe { super::bcm2835_spi_transfer(value as uint8_t) }
}

pub fn set_bit_order(order: BitOrder) {
    unsafe { super::bcm2835_spi_setBitOrder(order as uint8_t) };
}

pub fn set_data_mode(mode: Mode) {
    unsafe { super::bcm2835_spi_setDataMode(mode as uint8_t) };
}

pub fn set_clock_divider(divider: ClockDivider) {
    unsafe { super::bcm2835_spi_setClockDivider(divider as uint16_t) };
}

pub fn chip_select(cs_pins: ChipSelect) {
    unsafe { super::bcm2835_spi_chipSelect(cs_pins as uint8_t) };
}

pub fn set_chip_select_polarity(cs_pin: ChipSelect, level: super::PinLevel) {
    unsafe { super::bcm2835_spi_setChipSelectPolarity(cs_pin as uint8_t, level.as_uint8_t()) };
}