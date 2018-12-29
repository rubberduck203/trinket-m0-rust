#![no_std]
#![no_main]

extern crate atsamd21_hal as hal;
extern crate panic_abort;
extern crate cortex_m_rt;

pub use hal::atsamd21e18a::*;
use hal::prelude::*;
pub use hal::*;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut pins = peripherals.PORT.split();

    // Trinket 13
    let mut red_led = pins.pa10.into_open_drain_output(&mut pins.port);

    let mut clocks = clock::GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let core = CorePeripherals::take().unwrap();
    let mut delay = delay::Delay::new(core.SYST, &mut clocks);

    loop {
        delay.delay_ms(300u16);
        red_led.set_high();
        delay.delay_ms(300u16);
        red_led.set_low();
    }
}
