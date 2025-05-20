use esp_idf_hal::delay::Delay;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals: Peripherals = Peripherals::take().unwrap();
    let mut delay: Delay = Delay::new(1000);
    let mut led: PinDriver<'_, esp_idf_hal::gpio::Gpio14, esp_idf_hal::gpio::Output> = PinDriver::output(peripherals.pins.gpio14).unwrap();

    loop {
        led.set_high().unwrap();
        delay.delay_ms(1000u32);
        led.set_low().unwrap();
        delay.delay_ms(500u32);
    }
}
