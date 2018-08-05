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