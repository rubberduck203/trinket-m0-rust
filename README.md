# Board support package for Adafruit Trinket M0

Heavily based on the [atsamd21-rs crate](https://github.com/wez/atsamd21-rs).
This project uses the [atsamd21-hal crate](https://docs.rs/atsamd21-hal/latest/atsamd21_hal/) to create a BSP for the [Adafruit Trinket M0](https://learn.adafruit.com/adafruit-trinket-m0-circuitpython-arduino?view=all).

This isn't quite a BSP yet.
Right now it's a blinky light hello world program.
That was the first step toward creating a nice abstraction for the Trinket M0.

## Building

The library this project is dependent on is currently stuck on an older nightly build.

https://github.com/wez/atsamd21-rs/pull/3

```bash
$ rustup install nightly	$ rustup install nightly-2018-05-16
$ rustup default nightly	$ rustup default nightly-2018-05-16
$ rustup target add thumbv6m-none-eabi
```

Once you've installed the toolchain, build by specifying the target.

```bash
cargo build --target=thumbv6m-none-eabi
```

`.cargo/config` sets up the linker.
It expects `arm-none-eabi` to be on the path.

This article has a ton of good information about Rust on ARM.

http://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html

The cortex_m_rt documentation is also useful.

https://docs.rs/cortex-m-rt/0.5.1/cortex_m_rt/#an-example


## Flashing

Install Bossa

```bash
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/trinket_m0.bin

bassash #interactive shell

connect /dev/cu.usbmodem1441 #whatever serial port you're on
erase
write target/thumbv6m-none-eabi/debug/trinket_m0.bin
verify target/thumbv6m-none-eabi/debug/trinket_m0.bin
reset
```

In theory the following command should work, but I've no success thus far.

```bash
bossac -e -w -v -R target/thumbv6m-none-eabi/debug/trinket_m0.bin
```

## Known issues

It seems that any delay longer than ~300ms causes the program to crash after a single flash.