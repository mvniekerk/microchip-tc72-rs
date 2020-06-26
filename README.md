# Rust Microchip TC72 Driver

[![Crates.io Version][crates-io-badge]][crates-io]
[![Crates.io Downloads][crates-io-download-badge]][crates-io-download]
![No Std][no-std-badge]

This is a platform agnostic Rust driver for the Microchip TC72
temperature sensor series, based on the
[`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

Tested with the following sensors:

- [TC72](https://www.sensirion.com/shtc1/)

Docs: https://docs.rs/microchip-tc72r-rs

## The Device

The  TC72  is  a  digital  temperature  sensor  capable  ofreading temperatures from -55°C to +125°C. 
This sensor features a serial interface that allows communication  with  a  host  controller  or  other  peripherals.
TheTC72 interface is compatible with the SPI protocol. 

## Status

- [x] Measure temperature
- [x] Get device identifier
- [x] Sleep / Wakeup commands

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


<!-- Badges -->
[crates-io]: https://crates.io/crates/microchip-tc72r-rs
[crates-io-badge]: https://img.shields.io/crates/v/microchip-tc72r-rs.svg?maxAge=3600
[crates-io-download]: https://crates.io/crates/microchip-tc72r-rs
[crates-io-download-badge]: https://img.shields.io/crates/d/microchip-tc72r-rs.svg?maxAge=3600
[no-std-badge]: https://img.shields.io/badge/no__std-yes-blue
