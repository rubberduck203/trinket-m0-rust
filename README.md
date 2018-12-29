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
The easiest way to get the required arm toolchain is to use the official arm repository and apt install.

```bash
sudo add-apt-repository ppa:team-gcc-arm-embedded/ppa -y
sudo apt update
sudo apt install gcc-arm-embedded
```

This article has a ton of good information about Rust on ARM.

http://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html

The cortex_m_rt documentation is also useful.

https://docs.rs/cortex-m-rt/0.5.1/cortex_m_rt/#an-example


## Flashing

### Bossa

Install Bossa

```bash
arm-none-eabi-objcopy -O binary \
  target/thumbv6m-none-eabi/debug/trinket_m0 \
  target/thumbv6m-none-eabi/debug/trinket_m0.bin

bossac -e -w -v -R -p /dev/cu.usbmodem1441 target/thumbv6m-none-eabi/debug/trinket_m0.bin
```

In other words..

```bash
bossac --erase --write --verify --reset --port /dev/cu.usbmodem1441 target/thumbv6-none-eabi/debug/trinket_m0.bin
```

If using >=1.9 of `bossac`, then you *must* specify the offset.

```bash
# 0x2000 is the memory address where the code section starts
bossac -e -w -v -R -p ttyACM0 -o 0x2000 target/thumbv6-none-eabi/debug/trinket_m0.bin
```

### UF2

Adafruit's bootloader supports "drag & drop" programming via UF2 files.

#### Install conversion script

```bash
sudo apt install python
mkdir ~/bin
cd ~/bin
wget https://raw.githubusercontent.com/Microsoft/uf2/master/utils/uf2conv.py
```

#### Flashing

```bash
uf2conv.py -c -o target/thumbv6-none-eabi/debug/trinket_m0.uf2 target/thumbv6-none-eabi/debug/trinket_m0.bin
cp target/thumbv6-none-eabi/debug/trinket_m0.uf2 /media/rubberduck/TRINKETBOOT/
```
