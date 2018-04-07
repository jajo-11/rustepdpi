# Rust E-Paper Display for Raspberry Pi

This is a small sample program for working with e-paper displays on the raspberry pi using rust.
Specifically, this program assumes the [2.13inch e-Paper HAT (B) by waveshare][1] but it should 
be easily adaptable to other e-Paper displays.

## Building

### Build Requirements
* Obviously you need [Rust][2]
* The [bcm2835][3] Library by Mike McCauley

### Building

Building should be as easy as running `cargo build` in the project root.
Or you can build and run the project directly with `cargo run`.

[1]: https://www.waveshare.com/wiki/2.13inch_e-Paper_HAT_(B)
[2]: https://www.rust-lang.org 
[3]: http://www.airspayce.com/mikem/bcm2835
