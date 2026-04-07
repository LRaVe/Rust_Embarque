use crate::bsp_ensea::{Gamepad, GamepadState, Button};




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
