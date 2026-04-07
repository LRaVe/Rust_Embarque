use embassy_stm32::gpio::{Output, Input, Level, Pull, Speed};
use embassy_stm32::timer::qei::{Qei, Config as QeiConfig};

pub struct BargraphPins {
    pub led7: Output<'static>,  // PB5
    pub led6: Output<'static>,  // PB14
    pub led5: Output<'static>,  // PB4
    pub led4: Output<'static>,  // PB15
    pub led3: Output<'static>,  // PB13
    pub led2: Output<'static>,  // PA8
    pub led1: Output<'static>,  // PB2
    pub led0: Output<'static>,  // PC7
}

impl BargraphPins {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            led7: Output::new(p.PB5, Level::Low, Speed::Low),
            led6: Output::new(p.PB14, Level::Low, Speed::Low),
            led5: Output::new(p.PB4, Level::Low, Speed::Low),
            led4: Output::new(p.PB15, Level::Low, Speed::Low),
            led3: Output::new(p.PB13, Level::Low, Speed::Low),
            led2: Output::new(p.PA8, Level::Low, Speed::Low),
            led1: Output::new(p.PB2, Level::Low, Speed::Low),
            led0: Output::new(p.PC7, Level::Low, Speed::Low),
        }
    }
}

pub struct Bargraph {
    pub pins: BargraphPins,
    pub min: u8,
    pub max: u8,
}

impl Bargraph {
    pub fn new(pins: BargraphPins) -> Self {
        Self { 
            pins,
            min: 0,
            max: 100,
        }
    }
}

// Gamepad Button enumeration
#[derive(Clone, Copy, Debug)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Center,
}

// Gamepad state structure
#[derive(Clone, Copy, Debug)]
pub struct GamepadState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

impl GamepadState {
    pub fn none() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            center: false,
        }
    }
}

// Gamepad pins structure
pub struct GamepadPins {
    pub up: Input<'static>,
    pub down: Input<'static>,
    pub left: Input<'static>,
    pub right: Input<'static>,
    pub center: Input<'static>,
}

impl GamepadPins {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            up: Input::new(p.PC8, Pull::Down),
            down: Input::new(p.PB11, Pull::Down),
            left: Input::new(p.PC6, Pull::Down),
            right: Input::new(p.PC9, Pull::Down),
            center: Input::new(p.PC5, Pull::Down),
        }
    }
}

// Gamepad driver
pub struct Gamepad {
    pins: GamepadPins,
}

impl Gamepad {
    pub fn new(pins: GamepadPins) -> Self {
        Self { pins }
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        // Buttons are active low (pressed = low)
        match button {
            Button::Up => self.pins.up.is_low(),
            Button::Down => self.pins.down.is_low(),
            Button::Left => self.pins.left.is_low(),
            Button::Right => self.pins.right.is_low(),
            Button::Center => self.pins.center.is_low(),
        }
    }

    pub fn poll(&self) -> GamepadState {
        GamepadState {
            up: self.is_pressed(Button::Up),
            down: self.is_pressed(Button::Down),
            left: self.is_pressed(Button::Left),
            right: self.is_pressed(Button::Right),
            center: self.is_pressed(Button::Center),
        }
    }
}

pub struct GPS {
    pub gps_en: Output<'static>,  // PB13
}

impl GPS {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            gps_en: Output::new(p.PB13, Level::Low, Speed::Low),
        }
    }
}


pub struct GpioOutputs {
    pub led: Output<'static>,  // PA5
}

impl GpioOutputs {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            led: Output::new(p.PA5, Level::Low, Speed::Low),
        }
    }
}

pub struct GpioInputs {
    pub blue_button: Input<'static>,  // PC13
}

impl GpioInputs {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            blue_button: Input::new(p.PC13, Pull::Up),
        }
    }
}

pub struct Magnetometre {
    pub status: Input<'static>,  // PC1
    pub int: Input<'static>,     // PB0
}

impl Magnetometre {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            status: Input::new(p.PC1, Pull::Up),
            int: Input::new(p.PB0, Pull::Up),
        }
    }
}


pub struct RotaryEncoder {
    pub qei: Qei<'static, embassy_stm32::peripherals::TIM2>,
    pub button: Input<'static>,  // PA15
    offset: u16,  // Offset to allow setting arbitrary positions
}

impl RotaryEncoder {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        let config = QeiConfig::default();
        let qei = Qei::new(p.TIM2, p.PA0, p.PA1, config);
        
        Self {
            qei,
            button: Input::new(p.PA15, Pull::Down),
            offset: 0,
        }
    }

    /// Get the current encoder position (Qei count + offset)
    pub fn position(&self) -> u16 {
        self.qei.count().wrapping_add(self.offset)
    }

    /// Set the encoder position to a specific value
    pub fn set_position(&mut self, position: u16) {
        // Set offset so that position() returns the desired value
        // position = qei.count() + offset => offset = position - qei.count()
        self.offset = position.wrapping_sub(self.qei.count());
    }

    /// Reset the encoder position to 0
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Check if the button is pressed
    pub fn is_button_pressed(&self) -> bool {
        self.button.is_low()
    }
}


pub struct Stepper {
    pub direction: Output<'static>,  // PA7
    pub ms1: Output<'static>,        // PA11
    pub ms2: Output<'static>,        // PB12
    pub enable: Output<'static>,     // PA12
    pub step: Output<'static>,       // PA6
}

impl Stepper {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            direction: Output::new(p.PA7, Level::Low, Speed::Low),
            ms1: Output::new(p.PA11, Level::Low, Speed::Low),
            ms2: Output::new(p.PB12, Level::Low, Speed::Low),
            enable: Output::new(p.PA12, Level::Low, Speed::Low),
            step: Output::new(p.PA6, Level::Low, Speed::Low),
        }
    }
}


pub struct USART1 {
    pub tx: Output<'static>,  // PA9
    pub rx: Input<'static>,   // PA10
}

impl USART1 {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            tx: Output::new(p.PA9, Level::High, Speed::Low),
            rx: Input::new(p.PA10, Pull::Up),
        }
    }
}

pub struct USART2 {
    pub tx: Output<'static>,  // PA2
    pub rx: Input<'static>,   // PA3
}

impl USART2 {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            tx: Output::new(p.PA2, Level::High, Speed::Low),
            rx: Input::new(p.PA3, Pull::Up),
        }
    }
}

pub struct SPI2 {
    pub sck: Output<'static>,   // PB10
    pub mosi: Output<'static>,  // PC3
    pub miso: Input<'static>,   // PC2
    pub cs: Output<'static>,    // PC0
}

impl SPI2 {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            sck: Output::new(p.PB10, Level::Low, Speed::Low),
            mosi: Output::new(p.PC3, Level::Low, Speed::Low),
            miso: Input::new(p.PC2, Pull::Up),
            cs: Output::new(p.PC0, Level::High, Speed::Low),
        }
    }
}


pub struct I2C1 {
    pub scl: Output<'static>,  // PB6
    pub sda: Output<'static>,  // PB7
}

impl I2C1 {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            scl: Output::new(p.PB6, Level::High, Speed::Low),
            sda: Output::new(p.PB7, Level::High, Speed::Low),
        }
    }
}

// Connecteur pins are reserved for flexible use (typically as GPIO or alternate functions)
pub struct Connecteur {
    // PC10, PC11, PC12, PB8, PB9, PD2 - reserved for various protocols
    // These can be configured dynamically based on application needs
}

pub struct Board {
    pub encoder: RotaryEncoder,
    pub bargraph: Bargraph,
    pub stepper: Stepper,
    // pub gps: GPS,
    // pub gpio_outputs: GPIO_Outputs,
    // pub gpio_inputs: GPIO_Inputs,
    // pub gamepad: Gamepad,
    // pub magnetometre: Magnetometre,
    // pub usart1: USART1,
    // pub usart2: USART2,
    // pub spi2: SPI2,
    // pub i2c1: I2C1,
    // pub connecteur: Connecteur,
}

impl Board {
    pub fn new() -> Self {
        let p = embassy_stm32::init(Default::default());
        
        // Initialize encoder
        let config = embassy_stm32::timer::qei::Config::default();
        let qei = embassy_stm32::timer::qei::Qei::new(p.TIM2, p.PA0, p.PA1, config);
        let encoder = RotaryEncoder {
            qei,
            button: Input::new(p.PA15, Pull::Down),
            offset: 0,
        };
        
        // Initialize bargraph
        let bargraph_pins = BargraphPins {
            led7: Output::new(p.PB5, Level::Low, Speed::Low),
            led6: Output::new(p.PB14, Level::Low, Speed::Low),
            led5: Output::new(p.PB4, Level::Low, Speed::Low),
            led4: Output::new(p.PB15, Level::Low, Speed::Low),
            led3: Output::new(p.PB13, Level::Low, Speed::Low),
            led2: Output::new(p.PA8, Level::Low, Speed::Low),
            led1: Output::new(p.PB2, Level::Low, Speed::Low),
            led0: Output::new(p.PC7, Level::Low, Speed::Low),
        };
        let bargraph = Bargraph::new(bargraph_pins);
        
        // Initialize stepper
        let stepper = Stepper {
            direction: Output::new(p.PA7, Level::Low, Speed::Low),
            ms1: Output::new(p.PA11, Level::Low, Speed::Low),
            ms2: Output::new(p.PB12, Level::Low, Speed::Low),
            enable: Output::new(p.PA12, Level::Low, Speed::Low),
            step: Output::new(p.PA6, Level::Low, Speed::Low),
        };
        
        Self {
            encoder,
            bargraph,
            stepper,
        }
    }
}
