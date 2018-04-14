extern crate libc;

use self::libc::{c_int, c_uint, uint16_t, uint8_t};

pub mod gpio;
pub mod spio;

#[link(name = "bcm2835")]
extern "C" {
    fn bcm2835_init() -> c_int; // returns 1 if successful 0 if not
    fn bcm2835_delay(delay_time: c_uint);

    fn bcm2835_gpio_write(pin: uint8_t, value: uint8_t);
    fn bcm2835_gpio_lev(pin: uint8_t) -> uint8_t;
    fn bcm2835_gpio_fsel(pin: uint8_t, mode: uint8_t);

    fn bcm2835_spi_begin() -> c_int; // returns 1 if successful 0 if not call spi_end later
    fn bcm2835_spi_end();
    fn bcm2835_spi_transfer(value: uint8_t) -> uint8_t; //currently no need for the return value
    fn bcm2835_spi_setBitOrder(order: uint8_t);
    fn bcm2835_spi_setDataMode(mode: uint8_t);
    fn bcm2835_spi_setClockDivider(divider: uint16_t);
    fn bcm2835_spi_chipSelect(cs: uint8_t);
    fn bcm2835_spi_setChipSelectPolarity(cs: uint8_t, active: uint8_t);

}

#[derive(PartialEq)]
pub enum PinLevel {
    HIGH,
    LOW,
}

impl PinLevel {
    fn as_uint8_t(&self) -> uint8_t {
        match *self {
            PinLevel::HIGH => 1,
            PinLevel::LOW => 0,
        }
    }
}


pub fn init() {
    let x: c_int = unsafe { bcm2835_init() };
    if x != 1 {
        panic!("Init bcm2835 failed with code {}", x);
    }
}

pub fn delay(delay_ms: u32) {
    unsafe { bcm2835_delay(delay_ms as c_uint) };
}
