use core::sync::atomic::{AtomicBool, Ordering};

use crate::bsp_ensea::RotaryEncoder;
use crate::bargraph::{BARGRAPH_LEVEL, BARGRAPH_SIGNAL};
use crate::stepper::{STEPPER_DIRECTION, STEPPER_SPEED};
use embassy_time::{Duration, Timer};

pub static RESET_ENCODER: AtomicBool = AtomicBool::new(false);

/// Structure to hold the state of the rotary encoder
#[derive(Clone, Copy, Debug)]
pub struct EncoderState {
    pub position: u16,
    pub button_pressed: bool,
}

impl EncoderState {
    pub fn none() -> Self {
        Self {
            position: 0,
            button_pressed: false,
        }
    }
}

impl PartialEq for EncoderState {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.button_pressed == other.button_pressed
    }
}

/// Get the current state of the encoder (position and button)
pub fn read_encoder_state(encoder: &RotaryEncoder) -> EncoderState {
    EncoderState {
        position: encoder.position(),
        button_pressed: encoder.is_button_pressed(),
    }
}

/// Check if the encoder button was pressed
pub fn button_pressed(encoder: &RotaryEncoder) -> bool {
    encoder.is_button_pressed()
}

/// Reset the encoder to the starting position
pub fn reset_position(encoder: &mut RotaryEncoder) {
    encoder.reset();
}

/// Set the encoder to a specific position
pub fn set_position(encoder: &mut RotaryEncoder, position: u16) {
    encoder.set_position(position);
}

/// Tâche encoder qui accumule les pas et les traduit en incréments de vitesse moteur.
/// Chaque pas d'encoder = +/- vitesse (selon la direction).
/// Elle met à jour :
/// - BARGRAPH_LEVEL avec la position normalisée
/// - STEPPER_SPEED avec la vitesse accumulée (0-5000 Hz)
/// - STEPPER_DIRECTION avec la direction du dernier mouvement
#[embassy_executor::task]
pub async fn encoder_task(mut encoder: RotaryEncoder) {
    let mut last_position: i32 = encoder.position() as i32;
    let mut current_speed: i32 = 0;
    let speed_increment: i32 = 50;  // +/- 50 Hz par pas d'encoder
    let max_speed: i32 = 5000;      // Vitesse max 5000 Hz
    let poll_interval = Duration::from_millis(100);
    
    loop {
        Timer::after(poll_interval).await;
        
        // Vérifier si réinitialisation demandée
        if RESET_ENCODER.load(Ordering::Relaxed) {
            encoder.set_position(0);  // Réinitialiser la position matérielle de l'encodeur
            last_position = 0;
            current_speed = 0;
            STEPPER_SPEED.store(0, Ordering::Relaxed);
            STEPPER_DIRECTION.store(true, Ordering::Relaxed);
            BARGRAPH_LEVEL.store(0, Ordering::Relaxed);
            BARGRAPH_SIGNAL.signal(());
            RESET_ENCODER.store(false, Ordering::Relaxed);
            defmt::println!("Emergency reset complete: encoder position reset to 0");
        }
        
        let current_position = encoder.position() as i32;
        let delta = current_position - last_position;
        
        if delta != 0 {
            // Chaque pas change la vitesse d'une certaine quantité
            let direction = delta > 0;  // Direction du mouvement
            let speed_delta = (delta.abs() as i32) * speed_increment;
            
            // Accumuler la vitesse avec clamping
            if direction {
                current_speed = (current_speed + speed_delta).min(max_speed);
            } else {
                current_speed = (current_speed - speed_delta).max(0);
            }
            
            // Mettre à jour le bargraph avec la position
            let normalized_position = ((current_position.abs() as u32) % 100).min(100);
            BARGRAPH_LEVEL.store(normalized_position, Ordering::Relaxed);
            BARGRAPH_SIGNAL.signal(());
            
            // Mettre à jour la vitesse et direction du moteur
            STEPPER_SPEED.store(current_speed as u32, Ordering::Relaxed);
            STEPPER_DIRECTION.store(direction, Ordering::Relaxed);
            
            last_position = current_position;
        }
    }
}



