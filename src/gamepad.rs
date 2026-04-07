use crate::bsp_ensea::{Gamepad, GamepadState, Button};
use crate::rotary_encoder::RESET_ENCODER;
use crate::stepper::STEPPER_SPEED;
use core::sync::atomic::Ordering;
use embassy_time::{Duration, Timer};

/// Vérifie si un bouton spécifique est pressé
pub fn is_button_pressed(gamepad: &Gamepad, button: Button) -> bool {
    gamepad.is_pressed(button)
}

/// Retourne le nombre de boutons pressés
pub fn count_pressed_buttons(gamepad: &Gamepad) -> u8 {
    let state = gamepad.poll();
    let mut count = 0;
    if state.up { count += 1; }
    if state.down { count += 1; }
    if state.left { count += 1; }
    if state.right { count += 1; }
    if state.center { count += 1; }
    count
}

/// Retourne true si au moins un bouton est pressé
pub fn any_button_pressed(gamepad: &Gamepad) -> bool {
    gamepad.poll() != GamepadState::none()
}

/// Implémente PartialEq pour GamepadState pour comparaison
impl PartialEq for GamepadState {
    fn eq(&self, other: &Self) -> bool {
        self.up == other.up
            && self.down == other.down
            && self.left == other.left
            && self.right == other.right
            && self.center == other.center
    }
}

/// Tâche button reset qui monitore le bouton central du gamepad
/// Appuyer sur le bouton central réinitialise l'encoder, arrête le moteur (3 sec), puis réinitialise tout
#[embassy_executor::task]
pub async fn button_reset_task(gamepad: Gamepad) {
    let mut button_was_pressed = false;
    let poll_interval = Duration::from_millis(50);
    let motor_stop_duration = Duration::from_secs(3);  // Arrête le moteur pendant 3 secondes
    
    loop {
        Timer::after(poll_interval).await;
        
        let state = gamepad.poll();
        
        if state.center && !button_was_pressed {
            // Front montant du bouton: utilisateur vient d'appuyer
            defmt::println!("Motor stop initiated...");
            
            // Arrêter le moteur immédiatement
            STEPPER_SPEED.store(0, Ordering::Relaxed);
            
            // Garder le moteur arrêté pendant la durée spécifiée
            Timer::after(motor_stop_duration).await;
            
            // Maintenant réinitialiser l'encoder et les contrôles
            RESET_ENCODER.store(true, Ordering::Relaxed);
            defmt::println!("Emergency reset activated!");
            
            button_was_pressed = true;
        } else if !state.center {
            button_was_pressed = false;
        }
    }
}
