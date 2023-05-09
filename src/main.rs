#![no_std]
#![no_main]

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Arc, Circle, PrimitiveStyle},
};
use embedded_hal::digital::v2::OutputPin;
use hal::{adc::Adc, clocks::*, watchdog::Watchdog, Sio};
use panic_halt as _;
use pimoroni_pico_explorer::entry;
use pimoroni_pico_explorer::{hal, pac, PicoExplorer, XOSC_CRYSTAL_FREQ};

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // Enable watchdog and clocks
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        p.XOSC,
        p.CLOCKS,
        p.PLL_SYS,
        p.PLL_USB,
        &mut p.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.system_clock.get_freq().to_Hz());

    // Enable adc
    let adc = Adc::new(p.ADC, &mut p.RESETS);
    // let mut temp_sense = adc.enable_temp_sensor();

    let sio = Sio::new(p.SIO);

    let (mut explorer, pins) = PicoExplorer::new(
        p.IO_BANK0,
        p.PADS_BANK0,
        sio.gpio_bank0,
        p.SPI0,
        adc,
        &mut p.RESETS,
        &mut delay,
    );

    let mut led = pins.led.into_push_pull_output();

    const LCDWIDTH: i32 = 240;
    const LCDHEIGHT: i32 = 240;
    const RADIUS: u32 = 100;

    let backdrop_style = PrimitiveStyle::with_fill(Rgb565::YELLOW);
    let backdrop = Circle::new(Point::new(0, 0), RADIUS * 2)
        .translate(Point::new(20, 20))
        .into_styled(backdrop_style);

    // Draw eyes
    let eye_radius: u32 = 20;
    // let eye_line_offset = Point::new(eye_radius as i32, 0);
    let eye_style = PrimitiveStyle::with_fill(Rgb565::BLACK);
    let eye_center1 = Point::new(LCDWIDTH as i32 / 2 - 40, LCDHEIGHT as i32 / 2 - 15);
    let eye_center2 = Point::new(LCDWIDTH as i32 / 2 + 40, LCDHEIGHT as i32 / 2 - 15);

    let eye1_circle = Circle::new(eye_center1, eye_radius).into_styled(eye_style);
    // let eye1_line = Line::new(eye_center1 + eye_line_offset, eye_center1 - eye_line_offset)
    //     .into_styled(eye_style);

    let eye2_circle = Circle::new(eye_center2, eye_radius).into_styled(eye_style);
    // let eye2_line = Line::new(eye_center2 + eye_line_offset, eye_center2 - eye_line_offset)
    //     .into_styled(eye_style);

    // Draw mouth
    let mouth_center = Point::new(LCDWIDTH as i32 / 2, LCDHEIGHT as i32 / 2 + 10);
    let mouth_radius = 25;
    let mouth_style = PrimitiveStyle::with_stroke(Rgb565::BLACK, 3);
    let mouth_start_angle = 210.0f32;
    let mouth_sweep = 120.0f32;
    let mouth_arc = Arc::new(
        mouth_center,
        mouth_radius,
        mouth_start_angle.deg(),
        mouth_sweep.deg(),
    )
    .into_styled(mouth_style);

    let mut even = true;
    loop {
        delay.delay_ms(500);

        // Set GPIO25 to be low
        led.set_low().unwrap();

        delay.delay_ms(500);

        // Set GPIO25 to be high
        led.set_high().unwrap();

        backdrop.draw(&mut explorer.screen).unwrap();
        mouth_arc.draw(&mut explorer.screen).unwrap();
        eye1_circle.draw(&mut explorer.screen).unwrap();
        eye2_circle.draw(&mut explorer.screen).unwrap();

        // if even {
        //     eye1_circle.draw(&mut explorer.screen).unwrap();
        //     eye2_line.draw(&mut explorer.screen).unwrap();
        // } else {
        //     eye1_line.draw(&mut explorer.screen).unwrap();
        //     eye2_circle.draw(&mut explorer.screen).unwrap();
        // }

        even = !even;
    }
}
