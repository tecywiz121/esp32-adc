use embedded_hal_0_2_7::adc::{OneShot as OneShot0_2_7, Channel as Channel0_2_7};

use embedded_hal_1_0_0::adc::nb::{OneShot as OneShot1_0_0, Channel as Channel1_0_0};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::adc::{self, PoweredAdc};
use esp_idf_hal::gpio::Pin;
use esp_idf_hal::prelude::*;

fn channel0_2_7<T, U>(_: &T) -> T::ID where T: Channel0_2_7<U> {
    T::channel()
}

fn main() {
    esp_idf_sys::link_patches();

    let p = Peripherals::take().unwrap();
    let mut pin = p.pins.gpio14.into_analog_atten_11db().unwrap();
    let config = adc::config::Config::new().calibration(true);
    let mut adc = PoweredAdc::new(p.adc2, config).unwrap();

    esp_idf_sys::esp!(unsafe {
        esp_idf_sys::gpio_set_pull_mode(pin.pin(), esp_idf_sys::gpio_pull_mode_t_GPIO_PULLUP_ONLY)
    })
    .unwrap();

    loop {
        let value = nb::block!(OneShot0_2_7::read(&mut adc, &mut pin)).unwrap();
        println!("Channel 0.2.7: {}", channel0_2_7(&pin));
        println!("Value   0.2.7: {}", value);

        let value = nb::block!(OneShot1_0_0::read(&mut adc, &mut pin)).unwrap();
        println!("Channel 1.0.0: {}", Channel1_0_0::channel(&pin));
        println!("Value   1.0.0: {}", value);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
