#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use rustembarque::bargraph::bargraph_task;
use rustembarque::rotary_encoder::encoder_task;
use rustembarque::stepper::{stepper_task, StepperController};
use rustembarque::gamepad::button_reset_task;
use rustembarque::bsp_ensea::Board;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let board = Board::new();
    
    let stepper_controller = StepperController::new(board.stepper);
    
    // Spawn all tasks
    spawner.spawn(encoder_task(board.encoder)).unwrap();
    spawner.spawn(bargraph_task(board.bargraph)).unwrap();
    spawner.spawn(stepper_task(stepper_controller)).unwrap();
    spawner.spawn(button_reset_task(board.gamepad)).unwrap();
    
    defmt::println!("All tasks spawned: encoder, bargraph, stepper, gamepad reset");
    
    // Main loop (keep executor running)
    loop {
        embassy_time::Timer::after(embassy_time::Duration::from_secs(10)).await;
        defmt::println!("System running...");
    }
}
