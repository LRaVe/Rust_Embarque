#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use rustembarque::bsp_ensea::Stepper;
use rustembarque::stepper::StepperController;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
	let p = embassy_stm32::init(Default::default());
	let stepper_hw = Stepper::new(p);
	let mut stepper = StepperController::new(stepper_hw);

	// Le driver est actif bas: low = activé, high = désactivé.
	stepper.enable();
	defmt::println!("Stepper example started");

    // Microstepping à 1/2
    stepper.set_microstepping(rustembarque::stepper::MicrosteppingMode::Eighth);
	// Rotation continue tant que le moteur reste activé.
	stepper.set_speed(5000, false).await;
}
