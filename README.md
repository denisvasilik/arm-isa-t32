# arm-isa-t32 [design draft]

This crate provides an API for Arm ISA T32 instructions, known as Thumb in 
pre-Armv8 architectures. It is a design draft and solely used for experimental 
purposes. This work is based on the efforts of the [Cortex-M team][team].

## Disclaimer

Descriptive comments in the source files are taken from the following documents:

* [Arm Architecture Reference Manual - Armv8, for Armv8-A architecture profile][arm_ref_0]
* [Arm Architecture Reference Manual - Armv7-A and Armv7-R edition][arm_ref_1]

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
[arm_ref_0]: https://static.docs.arm.com/ddi0487/ea/DDI0487E_a_armv8_arm.pdf?_ga=2.140812338.1842257971.1570891190-1654016064.1570257337
[arm_ref_1]: https://static.docs.arm.com/ddi0406/cd/DDI0406C_d_armv7ar_arm.pdf?_ga=2.140812338.1842257971.1570891190-1654016064.1570257337
