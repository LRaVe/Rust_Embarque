use core::sync::atomic::{AtomicU32, Ordering};

use crate::bsp_ensea::Bargraph;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

pub static BARGRAPH_LEVEL: AtomicU32 = AtomicU32::new(0);
pub static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub struct BargraphController;

impl BargraphController {
    pub async fn wait_and_update() -> u32 {
        BARGRAPH_SIGNAL.wait().await;
        let value = BARGRAPH_LEVEL.load(Ordering::Relaxed);
        BARGRAPH_SIGNAL.reset();
        value
    }

    pub fn update_value(new_value: u32) {
        BARGRAPH_LEVEL.store(new_value, Ordering::Relaxed);
        BARGRAPH_SIGNAL.signal(());
    }

    pub fn apply_value(bargraph: &mut Bargraph, value: u32) {
        let value = value.min(u8::MAX as u32) as u8;

        let normalized = if value < bargraph.min {
            0
        } else if value > bargraph.max {
            100
        } else {
            ((value - bargraph.min) as u16 * 100 / (bargraph.max - bargraph.min) as u16) as u8
        };

        let num_leds = (normalized as usize * 8) / 100;

        for i in 0..8 {
            Self::set_led(bargraph, i, i < num_leds);
        }
    }

    fn set_led(bargraph: &mut Bargraph, index: usize, state: bool) {
        if state {
            match index {
                0 => bargraph.pins.led0.set_high(),
                1 => bargraph.pins.led1.set_high(),
                2 => bargraph.pins.led2.set_high(),
                3 => bargraph.pins.led3.set_high(),
                4 => bargraph.pins.led4.set_high(),
                5 => bargraph.pins.led5.set_high(),
                6 => bargraph.pins.led6.set_high(),
                7 => bargraph.pins.led7.set_high(),
                _ => (),
            }
        } else {
            match index {
                0 => bargraph.pins.led0.set_low(),
                1 => bargraph.pins.led1.set_low(),
                2 => bargraph.pins.led2.set_low(),
                3 => bargraph.pins.led3.set_low(),
                4 => bargraph.pins.led4.set_low(),
                5 => bargraph.pins.led5.set_low(),
                6 => bargraph.pins.led6.set_low(),
                7 => bargraph.pins.led7.set_low(),
                _ => (),
            }
        }
    }
}

/// Tâche bargraph qui attend les mises à jour du signal et affiche les valeurs.
#[embassy_executor::task]
pub async fn bargraph_task(mut bargraph: Bargraph) {
    loop {
        let value = BargraphController::wait_and_update().await;
        BargraphController::apply_value(&mut bargraph, value);
    }
}