use std::borrow::Borrow;

use esp_idf_hal::{
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
    prelude::*,
};
struct LED<'a> {
    driver: LedcDriver<'a>,
    value: u32,
    max_duty: u32,
}

impl<'a> LED<'a> {
    fn new(driver: LedcDriver<'a>) -> Self {
        let max_duty = driver.get_max_duty();
        Self {
            driver: driver,
            value: 0,
            max_duty,
        }
    }

    fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    fn on(&mut self) {
        self.driver
            .set_duty(self.max_duty * self.value / 255)
            .unwrap();
    }

    fn off(&mut self) {
        self.driver.set_duty(0).unwrap();
    }
}

#[derive(PartialEq)]
enum State {
    On,
    Off,
}
pub(crate) struct RGB<'a> {
    state: State,
    red: LED<'a>,
    green: LED<'a>,
    blue: LED<'a>,
}

impl RGB<'_> {
    pub fn new() -> Self {
        let peripherals = Peripherals::take().unwrap();

        let config = TimerConfig::default().frequency(25.kHz().into());
        let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &config).unwrap();
        let red_driver = LedcDriver::new(
            peripherals.ledc.channel0,
            timer.borrow(),
            peripherals.pins.gpio3,
            &config,
        );

        let green_driver = LedcDriver::new(
            peripherals.ledc.channel1,
            timer.borrow(),
            peripherals.pins.gpio4,
            &config,
        );

        let blue_driver = LedcDriver::new(
            peripherals.ledc.channel2,
            timer.borrow(),
            peripherals.pins.gpio5,
            &config,
        );

        Self {
            state: State::Off,
            red: LED::new(red_driver.unwrap()),
            green: LED::new(green_driver.unwrap()),
            blue: LED::new(blue_driver.unwrap()),
        }
    }

    pub fn on(&mut self) {
        if self.state == State::Off {
            self.state = State::On;
            self.red.on();
            self.green.on();
            self.blue.on();
        }
    }

    pub fn off(&mut self) {
        if self.state == State::On {
            self.state = State::Off;
            self.red.off();
            self.green.off();
            self.blue.off();
        }
    }

    fn update(&mut self) {
        if self.state == State::On {
            self.red.on();
            self.green.on();
            self.blue.on();
        }
    }

    pub fn set(&mut self, red: u32, green: u32, blue: u32) {
        self.red.set_value(red);
        self.green.set_value(green);
        self.blue.set_value(blue);
        self.update();
    }
}
