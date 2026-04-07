#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use embassy_time::{Duration, Timer};
use rustembarque::bargraph::{BargraphController, bargraph_task};
use rustembarque::bsp_ensea::Bargraph;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_stm32::init(Default::default());
    let bargraph_pins = rustembarque::bsp_ensea::BargraphPins::new(p);
    let mut bargraph = Bargraph::new(bargraph_pins);
    
    // Configure range
    bargraph.min = 10;
    bargraph.max = 90;

    // Spawn the bargraph task
    spawner.spawn(bargraph_task(bargraph)).unwrap();

    // Main loop: update bargraph value every ~500ms
    let mut counter: u32 = 0;
    loop {
        BargraphController::update_value(counter);
        Timer::after(Duration::from_millis(500)).await;
        counter = (counter + 10) % 100;
    }
}
