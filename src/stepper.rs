use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::bsp_ensea::Stepper;
use embassy_time::{Duration, Timer};

// Statics pour communication inter-tâche
pub static STEPPER_SPEED: AtomicU32 = AtomicU32::new(0);
pub static STEPPER_DIRECTION: AtomicBool = AtomicBool::new(true);

#[derive(Clone, Copy, Debug)]
pub enum MicrosteppingMode {
    Full,
    Half,
    Quarter,
    Eighth,
}


///Driver pour contrôler un moteur pas à pas
pub struct StepperController {
    stepper: Stepper,
}

impl StepperController {
    /// Crée un nouveau contrôleur de moteur pas à pas
    pub fn new(stepper: Stepper) -> Self {
        Self { stepper }
    }

    /// Active le moteur pas à pas
    pub fn enable(&mut self) {
        self.stepper.enable.set_low();
    }

    /// Désactiver le moteur pas à pas
    pub fn disable(&mut self) {
        self.stepper.enable.set_high();
    }

    /// Définir la direction du moteur pas à pas
    pub fn set_direction(&mut self, direction: bool) {
        if direction {
            self.stepper.direction.set_high();
        } else {
            self.stepper.direction.set_low();
        }
    }

    /// Génère une impulsion sur STEP avec temporisation haute et basse.
    pub async fn pulse(&mut self, high_time_us: u64, low_time_us: u64) {
        self.stepper.step.set_high();
        Timer::after(Duration::from_micros(high_time_us)).await;
        self.stepper.step.set_low();
        Timer::after(Duration::from_micros(low_time_us)).await;
    }

    /// Génère `steps` impulsions à la fréquence `speed_hz` dans la direction demandée.
    pub async fn set_speed(&mut self, speed_hz: u32, direction: bool) {
        if speed_hz == 0 {
            return;
        }

        if direction {
            self.stepper.direction.set_high();
        } else {
            self.stepper.direction.set_low();
        }

        let period_us = 1_000_000u64 / speed_hz as u64;
        let high_time_us = (period_us / 2).max(1);
        let low_time_us = period_us.saturating_sub(high_time_us).max(1);

        loop {
            if !self.stepper.enable.is_set_low() {
                break; // Arrêter si le moteur est désactivé
            }
            self.pulse(high_time_us, low_time_us).await;
        }
    }

    pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
        match mode {
            MicrosteppingMode::Full => {
                self.stepper.ms1.set_low();
                self.stepper.ms2.set_low();
            }
            MicrosteppingMode::Half => {
                self.stepper.ms1.set_high();
                self.stepper.ms2.set_low();
            }
            MicrosteppingMode::Quarter => {
                self.stepper.ms1.set_low();
                self.stepper.ms2.set_high();
            }
            MicrosteppingMode::Eighth => {
                self.stepper.ms1.set_high();
                self.stepper.ms2.set_high();
            }
        }
    }
}

/// Tâche stepper qui lit les statics STEPPER_SPEED et STEPPER_DIRECTION
/// et contrôle le moteur pas à pas en conséquence.
/// Génère les pulses une par une pour permettre les mises à jour dynamiques.
#[embassy_executor::task]
pub async fn stepper_task(mut stepper: StepperController) {
    stepper.enable();
    let mut last_speed = 0u32;
    
    loop {
        let speed = STEPPER_SPEED.load(Ordering::Relaxed);
        let direction = STEPPER_DIRECTION.load(Ordering::Relaxed);
        
        if speed > 0 {
            // Réactiver le moteur s'il était désactivé
            if last_speed == 0 {
                stepper.enable();
            }
            
            // Mettre à jour la direction
            stepper.set_direction(direction);
            
            // Générer une pulse à la fréquence demandée
            let period_us = 1_000_000u64 / speed as u64;
            let high_time_us = (period_us / 2).max(1);
            let low_time_us = period_us.saturating_sub(high_time_us).max(1);
            
            stepper.pulse(high_time_us, low_time_us).await;
        } else {
            // Désactiver seulement si c'était actif
            if last_speed > 0 {
                stepper.disable();
            }
            Timer::after(Duration::from_millis(100)).await;
        }
        
        last_speed = speed;
    }
}