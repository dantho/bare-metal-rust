#![no_std]
#![no_main]

use esp_rust_board::{
    esp32c3_hal::{
        prelude::*,
        peripherals::Peripherals,
        clock::ClockControl,
        pulse_control::ClockSource,
        i2c::I2C,
        timer::TimerGroup,
        Delay, Rtc, PulseControl, IO,
        utils::{smartLedAdapter, SmartLedsAdapter},
    },
    esp_backtrace as _,
    icm42670::{
        Address,
        Icm42670,
        accelerometer::{
            Accelerometer,
            vector::{
                VectorExt,
                F32x3,
            },
            orientation::{
                Orientation,
                Orientation::*
            },
            Tracker,
        },
    },
    print, println,
    shared_bus::BusManagerSimple,
    shtcx::{shtc3, LowPower, PowerMode},
    smart_leds::{
        RGB8,
        brightness, gamma,
        SmartLedsWrite,
        colors,
    },
};
use owo_colors::OwoColorize;
use micromath::{F32Ext};

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Configure RMT peripheral globally
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
        ClockSource::APB,
        0,
        0,
        0,
    )
    .unwrap();

    // We use one of the RMT channels to instantiate a `SmartLedsAdapter` which can
    // be used directly with all `smart_led` implementations
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = <smartLedAdapter!(1)>::new(pulse.channel0, io.pins.gpio2);

    // Initialize the Delay peripheral and use it to modify the LED in a loop.
    let mut delay = Delay::new(&clocks);

    // Initialize the I2C bus using GPIO10 for SDA and GPIO8 for SCL, running at
    // 400kHz.
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio10,
        io.pins.gpio8,
        400u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    // Create a bus manager so that we can share the I2C bus between sensor drivers
    // while avoiding ownership issues.
    let bus = BusManagerSimple::new(i2c);
    let mut icm = Icm42670::new(bus.acquire_i2c(), Address::Primary).unwrap();
    let mut sht = shtc3(bus.acquire_i2c());

    // The SHTC3 temperature/humidity sensor must be woken up prior to reading.
    sht.wakeup(&mut delay).unwrap();

    // Setup Accel orientation tracker
    let mut tracker = Tracker::new(0.5);
    let mut prior_orient = Orientation::Unknown;
    
    loop {
        // Read and display normalized accelerometer and gyroscope values.
        let accel_norm = icm.accel_norm().unwrap();
        let gyro_norm = icm.gyro_norm().unwrap();
        tracker.update(accel_norm);
        let orient = tracker.orientation();
        // Correct FaceUp polarity
        let orient = match orient {
            Orientation::FaceUp => Orientation::FaceDown,
            Orientation::FaceDown => Orientation::FaceUp,
            other => other,
        };
        if orient != prior_orient {
            prior_orient = orient;
            println!("{:?}", orient);
            let orient_color = directional_color(orient);
            led.write(brightness(gamma(core::iter::once(orient_color)), 50)).unwrap();
        }

        // print!(
        //     "ACCEL = X: {:+.02} Y: {:+.02} Z: {:+.02}   ",
        //     accel_norm.x, accel_norm.y, accel_norm.z
        // );

        // if accel_norm.magnitude() > 1.2 {
        //     print!(
        //         "Mag: {:+.02}   ",
        //         accel_norm.magnitude()
        //     );
        // }

        // print!(
        //     "GYRO = X: {:+.02} Y: {:+.02} Z: {:+.02}   ",
        //     gyro_norm.x, gyro_norm.y, gyro_norm.z
        // );

        // // Read and display temperature and relative humidity values.
        // let measurement = sht.measure(PowerMode::NormalMode, &mut delay).unwrap();
        // let temp_c = measurement.temperature.as_degrees_celsius();
        // let temp_f = c2f(temp_c);
        // if temp_f > 80.0 {
        //     print!("TEMP = {:+.2} °C {:+.2} °F   ", temp_c.red(), temp_f.red());
        // } else {
        //     print!("TEMP = {:+.2} °C {:+.2} °F   ", temp_c, temp_f);
        // };
        // println!("RH = {:+.2} %RH", measurement.humidity.as_percent());

        // // Foreground colors
        // println!("My number is {:#x}!", 10.green());
        // // Background colors
        // println!("My number is not {}!", 4.on_red());

        delay.delay_ms(1000u32);
    }
}

fn c2f(temp_c: f32) -> f32 {
    temp_c/5.0*9.0+32.0
}

fn directional_color(dir: Orientation) -> RGB8 {
    match dir {
        Unknown => colors::BLACK,
        FaceUp => RGB8::new(0x7F,0x0F,0xCF),
        FaceDown => RGB8::new(0x0,0x6F,0x1F),
        PortraitUp => colors::BLUE,
        PortraitDown => RGB8::new(0x3F,0x5F,0xCF),
        LandscapeUp => RGB8 { r: 0x90, g: 0xB0, b: 0x00 }, // RGB8::new(0x3F,0x3F,0x3F),
        LandscapeDown => RGB8::new(0x6F,0x9F,0xC0),
    }
}
