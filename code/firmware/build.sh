#!/bin/bash
#rustup override set nightly-2022-01-17
#rustup component add rust-src --toolchain nightly-2022-01-17-x86_64-unknown-linux-gnu
rustup override set nightly-2021-01-07
rustup component add rust-src --toolchain nightly-2021-01-07-x86_64-unknown-linux-gnu
cargo build && avrdude -patmega328p -carduino -P /dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/firmware.elf:e