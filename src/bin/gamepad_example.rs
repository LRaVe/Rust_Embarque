#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm;
use rustembarque::bsp_ensea::{Gamepad, GamepadPins, Button};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let gamepad_pins = GamepadPins::new(p);
    let gamepad = Gamepad::new(gamepad_pins);

    defmt::println!("Gamepad Example Started");

    loop {
        // Poll the gamepad to get all button states
        let state = gamepad.poll();

        // Display button states via defmt
        defmt::println!(
            "Gamepad: UP={}, DOWN={}, LEFT={}, RIGHT={}, CENTER={}",
            state.up,
            state.down,
            state.left,
            state.right,
            state.center
        );

        // Alternative: check individual buttons
        if gamepad.is_pressed(Button::Up) {
            defmt::println!("UP button pressed!");
        }
        if gamepad.is_pressed(Button::Down) {
            defmt::println!("DOWN button pressed!");
        }
        if gamepad.is_pressed(Button::Left) {
            defmt::println!("LEFT button pressed!");
        }
        if gamepad.is_pressed(Button::Right) {
            defmt::println!("RIGHT button pressed!");
        }
        if gamepad.is_pressed(Button::Center) {
            defmt::println!("CENTER button pressed!");
        }

        // Delay for readability
        for _ in 0..10000 {
            asm::nop();
        }
    }
}
