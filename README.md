# Board support package for Adafruit Trinket M0

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