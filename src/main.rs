use embedded_hal::adc::nb::OneShot;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::prelude::*;
use esp_idf_hal::adc::{self, PoweredAdc};
use esp_idf_hal::gpio::Pin;

fn main() {
    esp_idf_sys::link_patches();

    let p = Peripherals::take().unwrap();
    let mut pin = p.pins.gpio14.into_analog_atten_11db().unwrap();
    let config = adc::config::Config::new().calibration(true);
    let mut adc = PoweredAdc::new(p.adc2, config).unwrap();

    unsafe {
        esp_idf_sys::gpio_set_pull_mode(pin.pin(), esp_idf_sys::gpio_pull_mode_t_GPIO_PULLUP_ONLY);
    }

    loop {
        let value = nb::block!(adc.read(&mut pin)).unwrap();
        println!("Value: {}", value);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
