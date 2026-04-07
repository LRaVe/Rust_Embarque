embassy_hal_internal::peripherals_definition!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD2,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    COMP1,
    COMP2,
    CRC,
    DAC1,
    DBGMCU,
    DMA1,
    DMA2,
    FLASH,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    LCD,
    LPTIM1,
    LPTIM2,
    LPUART1,
    OPAMP1,
    OPAMP2,
    PWR,
    QUADSPI,
    MCO,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SDMMC1,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TIM1,
    TIM15,
    TIM16,
    TIM17,
    TIM2,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TSC,
    UART4,
    UART5,
    UID,
    USART1,
    USART2,
    USART3,
    USB_OTG_FS,
    VREFBUF,
    VREFINTCAL,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::peripherals_struct!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD2,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    COMP1,
    COMP2,
    CRC,
    DAC1,
    DBGMCU,
    DMA1,
    DMA2,
    FLASH,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    LCD,
    LPTIM1,
    LPTIM2,
    LPUART1,
    OPAMP1,
    OPAMP2,
    PWR,
    QUADSPI,
    MCO,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SDMMC1,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TIM1,
    TIM15,
    TIM16,
    TIM17,
    TIM2,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TSC,
    UART4,
    UART5,
    UID,
    USART1,
    USART2,
    USART3,
    USB_OTG_FS,
    VREFBUF,
    VREFINTCAL,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD_PVM,
    TAMP_STAMP,
    RTC_WKUP,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_CHANNEL1,
    DMA1_CHANNEL2,
    DMA1_CHANNEL3,
    DMA1_CHANNEL4,
    DMA1_CHANNEL5,
    DMA1_CHANNEL6,
    DMA1_CHANNEL7,
    ADC1_2,
    CAN1_TX,
    CAN1_RX0,
    CAN1_RX1,
    CAN1_SCE,
    EXTI9_5,
    TIM1_BRK_TIM15,
    TIM1_UP_TIM16,
    TIM1_TRG_COM_TIM17,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    DFSDM1_FLT3,
    TIM8_BRK,
    TIM8_UP,
    TIM8_TRG_COM,
    TIM8_CC,
    ADC3,
    FMC,
    SDMMC1,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6_DAC,
    TIM7,
    DMA2_CHANNEL1,
    DMA2_CHANNEL2,
    DMA2_CHANNEL3,
    DMA2_CHANNEL4,
    DMA2_CHANNEL5,
    DFSDM1_FLT0,
    DFSDM1_FLT1,
    DFSDM1_FLT2,
    COMP,
    LPTIM1,
    LPTIM2,
    OTG_FS,
    DMA2_CHANNEL6,
    DMA2_CHANNEL7,
    LPUART1,
    QUADSPI,
    I2C3_EV,
    I2C3_ER,
    SAI1,
    SAI2,
    SWPMI1,
    TSC,
    LCD,
    RNG,
    FPU,
);
pub const MAX_ERASE_SIZE: usize = 2048u32 as usize;
pub mod flash_regions {
    impl crate::flash::FlashBank {
        #[doc = r" Absolute base address."]
        pub fn base(&self) -> u32 {
            match self {
                crate::flash::FlashBank::Bank1 => 134217728u32,
                crate::flash::FlashBank::Bank2 => 134742016u32,
                crate::flash::FlashBank::Otp => panic!("OTP not present"),
            }
        }
    }
    pub const BANK1_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        offset: 0u32,
        size: 524288u32,
        erase_size: 2048u32,
        write_size: 8u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK2_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank2,
        offset: 0u32,
        size: 524288u32,
        erase_size: 2048u32,
        write_size: 8u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank2Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region: Bank1Region<'d, MODE>,
        pub bank2_region: Bank2Region<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(p: embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>) -> Self {
            Self {
                bank1_region: Bank1Region(
                    &BANK1_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank2_region: Bank2Region(
                    &BANK2_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 2usize] = [&BANK1_REGION, &BANK2_REGION];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pll1_q")
            },
            crate::pac::rcc::vals::Adcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "sys")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 13u8)),
            (19u8, 13u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pll1_q")
            },
            crate::pac::rcc::vals::Adcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "sys")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 13u8)),
            (19u8, 13u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "pll1_q")
            },
            crate::pac::rcc::vals::Adcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "sys")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 13u8)),
            (19u8, 13u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 25u8)),
            (22u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 12u8)),
            (18u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::DAC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 29u8)),
            (22u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DAC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 0u8)),
            (18u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 1u8)),
            (18u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FLASH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 8u8)),
            (18u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FLASH {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().i2c1sel() {
            crate::pac::rcc::vals::I2c1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
            },
            crate::pac::rcc::vals::I2c1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "sys")
            },
            crate::pac::rcc::vals::I2c1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 21u8)),
            (22u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().i2c2sel() {
            crate::pac::rcc::vals::I2c2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
            },
            crate::pac::rcc::vals::I2c2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "sys")
            },
            crate::pac::rcc::vals::I2c2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 22u8)),
            (22u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().i2c3sel() {
            crate::pac::rcc::vals::I2c3sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
            },
            crate::pac::rcc::vals::I2c3sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "sys")
            },
            crate::pac::rcc::vals::I2c3sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 23u8)),
            (22u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LCD {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LCD" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LCD" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 9u8)),
            (22u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LCD {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lptim1sel() {
            crate::pac::rcc::vals::Lptim1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
            },
            crate::pac::rcc::vals::Lptim1sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lsi")
            },
            crate::pac::rcc::vals::Lptim1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "hsi")
            },
            crate::pac::rcc::vals::Lptim1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 31u8)),
            (22u8, 31u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lptim2sel() {
            crate::pac::rcc::vals::Lptim2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk1")
            },
            crate::pac::rcc::vals::Lptim2sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lsi")
            },
            crate::pac::rcc::vals::Lptim2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "hsi")
            },
            crate::pac::rcc::vals::Lptim2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((15u8, 5u8)),
            (23u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPUART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lpuart1sel() {
            crate::pac::rcc::vals::Lpuart1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk1")
            },
            crate::pac::rcc::vals::Lpuart1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "sys")
            },
            crate::pac::rcc::vals::Lpuart1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "hsi")
            },
            crate::pac::rcc::vals::Lpuart1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPUART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((15u8, 0u8)),
            (23u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPUART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::OPAMP1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 30u8)),
            (22u8, 30u8),
            Some(1u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::OPAMP1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::OPAMP2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 30u8)),
            (22u8, 30u8),
            Some(1u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::OPAMP2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::PWR {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 28u8)),
            (22u8, 28u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::PWR {}
impl crate::rcc::SealedRccPeripheral for peripherals::QUADSPI {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 8u8)),
            (20u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::QUADSPI {}
impl crate::rcc::SealedRccPeripheral for peripherals::RNG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "hclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 18u8)),
            (19u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RNG {}
impl crate::rcc::SealedRccPeripheral for peripherals::RTC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.bdcr().read().rtcsel() {
            crate::pac::rcc::vals::Rtcsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lse")
            },
            crate::pac::rcc::vals::Rtcsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lsi")
            },
            crate::pac::rcc::vals::Rtcsel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "RTC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (22u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Standby,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RTC {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().sai1sel() {
            crate::pac::rcc::vals::Sai1sel::PLLSAI1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pllsai1_p")
            },
            crate::pac::rcc::vals::Sai1sel::PLLSAI2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pllsai2_p")
            },
            crate::pac::rcc::vals::Sai1sel::PLL1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pll1_p")
            },
            crate::pac::rcc::vals::Sai1sel::SAI1_EXTCLK => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sai1_extclk . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "sai1_extclk")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 21u8)),
            (24u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().sai2sel() {
            crate::pac::rcc::vals::Sai2sel::PLLSAI1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pllsai1_p")
            },
            crate::pac::rcc::vals::Sai2sel::PLLSAI2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pllsai2_p")
            },
            crate::pac::rcc::vals::Sai2sel::PLL1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pll1_p")
            },
            crate::pac::rcc::vals::Sai2sel::SAI2_EXTCLK => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sai2_extclk . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "sai2_extclk")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 22u8)),
            (24u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SDMMC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "hsi48")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pllsai1_q")
            },
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pll1_q")
            },
            crate::pac::rcc::vals::Clk48sel::MSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . msi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "msi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SDMMC1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 10u8)),
            (24u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SDMMC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 12u8)),
            (24u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 14u8)),
            (22u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 15u8)),
            (22u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SYSCFG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 0u8)),
            (24u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SYSCFG {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 11u8)),
            (24u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM15 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 16u8)),
            (24u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM15 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM16 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 17u8)),
            (24u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM16 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM17 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 18u8)),
            (24u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM17 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 0u8)),
            (22u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 1u8)),
            (22u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 2u8)),
            (22u8, 2u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 3u8)),
            (22u8, 3u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 4u8)),
            (22u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM7 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 5u8)),
            (22u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM8 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 13u8)),
            (24u8, 13u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM8 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TSC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TSC" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TSC" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 16u8)),
            (18u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TSC {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART4 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().uart4sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART4"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 19u8)),
            (22u8, 19u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART5 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().uart5sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART5"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 20u8)),
            (22u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart1sel() {
            crate::pac::rcc::vals::Usart1sel::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
            },
            crate::pac::rcc::vals::Usart1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "sys")
            },
            crate::pac::rcc::vals::Usart1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "hsi")
            },
            crate::pac::rcc::vals::Usart1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((16u8, 14u8)),
            (24u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart2sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 17u8)),
            (22u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart3sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((14u8, 18u8)),
            (22u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_FS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "hsi48")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pllsai1_q")
            },
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pll1_q")
            },
            crate::pac::rcc::vals::Clk48sel::MSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . msi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "msi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_FS"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 12u8)),
            (19u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_FS {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (22u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG {}
pub(crate) static mut REFCOUNTS: [u8; 2usize] = [0u8, 0u8];
pub mod mux {
    pub use crate::pac::rcc::vals::Adcsel;
    pub use crate::pac::rcc::vals::Clk48sel;
    pub use crate::pac::rcc::vals::I2c1sel;
    pub use crate::pac::rcc::vals::I2c2sel;
    pub use crate::pac::rcc::vals::I2c3sel;
    pub use crate::pac::rcc::vals::Lptim1sel;
    pub use crate::pac::rcc::vals::Lptim2sel;
    pub use crate::pac::rcc::vals::Lpuart1sel;
    pub use crate::pac::rcc::vals::Rtcsel;
    pub use crate::pac::rcc::vals::Sai1sel;
    pub use crate::pac::rcc::vals::Sai2sel;
    pub use crate::pac::rcc::vals::Usart1sel;
    pub use crate::pac::rcc::vals::Usartsel;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {
        pub rtcsel: Rtcsel,
        pub adcsel: Adcsel,
        pub clk48sel: Clk48sel,
        pub i2c1sel: I2c1sel,
        pub i2c2sel: I2c2sel,
        pub i2c3sel: I2c3sel,
        pub lptim1sel: Lptim1sel,
        pub lptim2sel: Lptim2sel,
        pub lpuart1sel: Lpuart1sel,
        pub sai1sel: Sai1sel,
        pub sai2sel: Sai2sel,
        pub uart4sel: Usartsel,
        pub uart5sel: Usartsel,
        pub usart1sel: Usart1sel,
        pub usart2sel: Usartsel,
        pub usart3sel: Usartsel,
    }
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {
            crate::pac::RCC.bdcr().modify(|w| {
                w.set_rtcsel(self.rtcsel);
            });
            crate::pac::RCC.ccipr().modify(|w| {
                w.set_adcsel(self.adcsel);
                w.set_clk48sel(self.clk48sel);
                w.set_i2c1sel(self.i2c1sel);
                w.set_i2c2sel(self.i2c2sel);
                w.set_i2c3sel(self.i2c3sel);
                w.set_lptim1sel(self.lptim1sel);
                w.set_lptim2sel(self.lptim2sel);
                w.set_lpuart1sel(self.lpuart1sel);
                w.set_sai1sel(self.sai1sel);
                w.set_sai2sel(self.sai2sel);
                w.set_uart4sel(self.uart4sel);
                w.set_uart5sel(self.uart5sel);
                w.set_usart1sel(self.usart1sel);
                w.set_usart2sel(self.usart2sel);
                w.set_usart3sel(self.usart3sel);
            });
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub hclk1: crate::time::MaybeHertz,
    pub hclk2: crate::time::MaybeHertz,
    pub hclk3: crate::time::MaybeHertz,
    pub hse: crate::time::MaybeHertz,
    pub hsi: crate::time::MaybeHertz,
    pub hsi48: crate::time::MaybeHertz,
    pub lse: crate::time::MaybeHertz,
    pub lsi: crate::time::MaybeHertz,
    pub msi: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pclk2: crate::time::MaybeHertz,
    pub pclk2_tim: crate::time::MaybeHertz,
    pub pll1_p: crate::time::MaybeHertz,
    pub pll1_q: crate::time::MaybeHertz,
    pub pllsai1_p: crate::time::MaybeHertz,
    pub pllsai1_q: crate::time::MaybeHertz,
    pub pllsai2_p: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sai1_extclk: crate::time::MaybeHertz,
    pub sai2_extclk: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
}
pub unsafe fn init_mdma() {}
pub unsafe fn init_dma() {}
pub unsafe fn init_bdma() {
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma1en(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma2en(true));
}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpioeen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiofen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiogen(true));
    crate::pac::RCC.ahb2enr().modify(|w| w.set_gpiohen(true));
}
impl_adc_pin!(ADC1, PA0, 5u8);
impl_adc_pin!(ADC1, PA1, 6u8);
impl_adc_pin!(ADC1, PA2, 7u8);
impl_adc_pin!(ADC1, PA3, 8u8);
impl_adc_pin!(ADC1, PA4, 9u8);
impl_adc_pin!(ADC1, PA5, 10u8);
impl_adc_pin!(ADC1, PA6, 11u8);
impl_adc_pin!(ADC1, PA7, 12u8);
impl_adc_pin!(ADC1, PB0, 15u8);
impl_adc_pin!(ADC1, PB1, 16u8);
impl_adc_pin!(ADC1, PC0, 1u8);
impl_adc_pin!(ADC1, PC1, 2u8);
impl_adc_pin!(ADC1, PC2, 3u8);
impl_adc_pin!(ADC1, PC3, 4u8);
impl_adc_pin!(ADC1, PC4, 13u8);
impl_adc_pin!(ADC1, PC5, 14u8);
impl_adc_pin!(ADC2, PA0, 5u8);
impl_adc_pin!(ADC2, PA1, 6u8);
impl_adc_pin!(ADC2, PA2, 7u8);
impl_adc_pin!(ADC2, PA3, 8u8);
impl_adc_pin!(ADC2, PA4, 9u8);
impl_adc_pin!(ADC2, PA5, 10u8);
impl_adc_pin!(ADC2, PA6, 11u8);
impl_adc_pin!(ADC2, PA7, 12u8);
impl_adc_pin!(ADC2, PB0, 15u8);
impl_adc_pin!(ADC2, PB1, 16u8);
impl_adc_pin!(ADC2, PC0, 1u8);
impl_adc_pin!(ADC2, PC1, 2u8);
impl_adc_pin!(ADC2, PC2, 3u8);
impl_adc_pin!(ADC2, PC3, 4u8);
impl_adc_pin!(ADC2, PC4, 13u8);
impl_adc_pin!(ADC2, PC5, 14u8);
impl_adc_pin!(ADC3, PC0, 1u8);
impl_adc_pin!(ADC3, PC1, 2u8);
impl_adc_pin!(ADC3, PC2, 3u8);
impl_adc_pin!(ADC3, PC3, 4u8);
pin_trait_impl!(
    crate::can::RxPin,
    CAN1,
    PA11,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN1,
    PA12,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::RxPin,
    CAN1,
    PB8,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN1,
    PB9,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
impl_comp_inm_pin!(COMP1, PB1, 0u8);
impl_comp_inp_pin!(COMP1, PB2, 0u8);
impl_comp_inm_pin!(COMP1, PC4, 0u8);
impl_comp_inp_pin!(COMP1, PC5, 0u8);
impl_comp_inm_pin!(COMP2, PB3, 0u8);
impl_comp_inp_pin!(COMP2, PB4, 0u8);
impl_comp_inp_pin!(COMP2, PB6, 0u8);
impl_comp_inm_pin!(COMP2, PB7, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch1>, DAC1, PA4, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch2>, DAC1, PA5, 0u8);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB6,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB7,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB8,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB9,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C2,
    PB10,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C2,
    PB11,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C2,
    PB13,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C2,
    PB14,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C3,
    PC0,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C3,
    PC1,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA1, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PA10, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA15, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA2, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA3, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA6, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PA7, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PA8, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PA9, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB0, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB1, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB10, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB11, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB12, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB13, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB14, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB15, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB3, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB4, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB5, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB7, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PB8, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PB9, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC0, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC1, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PC10, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC10, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PC11, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC11, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PC12, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC12, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC2, 11u8);
pin_trait_impl!(crate::lcd::VlcdPin, LCD, PC3, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC4, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC5, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC6, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC7, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC8, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PC9, 11u8);
pin_trait_impl!(crate::lcd::ComPin, LCD, PD2, 11u8);
pin_trait_impl!(crate::lcd::SegPin, LCD, PD2, 11u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PB2, 1u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PC1, 1u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PA4, 14u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PA8, 14u8);
pin_trait_impl!(
    crate::usart::RxPin,
    LPUART1,
    PB10,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    LPUART1,
    PB11,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    LPUART1,
    PB12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    LPUART1,
    PB12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    LPUART1,
    PB13,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    LPUART1,
    PC0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    LPUART1,
    PC1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
impl_opamp_vp_pin!(OPAMP1, PA0, 0u8);
impl_opamp_bias_pin!(OPAMP1, PA1, 0u8);
impl_opamp_vn_pin!(OPAMP1, PA1, 0u8);
impl_opamp_vout_pin!(OPAMP1, PA3);
impl_opamp_external_output!(OPAMP1, ADC1, 8u8);
impl_opamp_external_output!(OPAMP1, ADC2, 8u8);
impl_opamp_vp_pin!(OPAMP2, PA6, 0u8);
impl_opamp_bias_pin!(OPAMP2, PA7, 0u8);
impl_opamp_vn_pin!(OPAMP2, PA7, 0u8);
impl_opamp_vout_pin!(OPAMP2, PB0);
impl_opamp_external_output!(OPAMP2, ADC1, 15u8);
impl_opamp_external_output!(OPAMP2, ADC2, 15u8);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PA6, 10u8);
pin_trait_impl!(crate::qspi::BK1D2Pin, QUADSPI, PA7, 10u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PB0, 10u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PB1, 10u8);
pin_trait_impl!(crate::qspi::SckPin, QUADSPI, PB10, 10u8);
pin_trait_impl!(crate::qspi::BK1NSSPin, QUADSPI, PB11, 10u8);
pin_trait_impl!(crate::rcc::McoPin, MCO, PA8, 0u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI1, PA4, 13u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI1, PB10, 13u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI1, PB3, 13u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI1, PB4, 13u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI1, PB5, 13u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI1, PB6, 13u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI1, PB8, 13u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI1, PB9, 13u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PC3, 13u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PA15, 13u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI2, PB12, 13u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI2, PB13, 13u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI2, PB14, 13u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI2, PB15, 13u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI2, PC10, 13u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PC11, 13u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PC12, 13u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI2, PC6, 13u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PC7, 13u8);
pin_trait_impl!(crate::sdmmc::D4Pin, SDMMC1, PB8, 12u8);
pin_trait_impl!(crate::sdmmc::D5Pin, SDMMC1, PB9, 12u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDMMC1, PC10, 12u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDMMC1, PC11, 12u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDMMC1, PC12, 12u8);
pin_trait_impl!(crate::sdmmc::D6Pin, SDMMC1, PC6, 12u8);
pin_trait_impl!(crate::sdmmc::D7Pin, SDMMC1, PC7, 12u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDMMC1, PC8, 12u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDMMC1, PC9, 12u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDMMC1, PD2, 12u8);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PA5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PA6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PA7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PB3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PB4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PB5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB10,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB13,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PB14,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PB15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PC2,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PC3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA15,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PB3,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PB4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PB5,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PC10,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PC11,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PC12,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM1,
    PA10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM1,
    PA11,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn2>,
    TIM1,
    PA11,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM1,
    PA11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM1,
    PA12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PA6,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM1,
    PA6,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PA7,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM1,
    PA8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM1,
    PA9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PB12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM1,
    PB12,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PB13,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB14,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM15,
    PA1,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM15,
    PA2,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM15,
    PA3,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM15,
    PA9,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM15,
    PB12,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM15,
    PB13,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM15,
    PB14,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM15,
    PB15,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM16,
    PA6,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM16,
    PB5,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM16,
    PB6,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM16,
    PB8,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM17,
    PA10,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM17,
    PA7,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM17,
    PB4,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM17,
    PB7,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM17,
    PB9,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA0,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PA1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA15,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PA2,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PA3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA5,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA5,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PB10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PB11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PB3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PA6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PA7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PB0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PB1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PB4,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PB5,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PC6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PC7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PC8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PC9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM3,
    PD2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM4,
    PB6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM4,
    PB7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM4,
    PB8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM4,
    PB9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM5,
    PA0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM5,
    PA1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM5,
    PA2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM5,
    PA3,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM8,
    PA0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA5,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM8,
    PA6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM8,
    PA6,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB1,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB14,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB15,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM8,
    PB6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn2>,
    TIM8,
    PB6,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM8,
    PB7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn1>,
    TIM8,
    PB7,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM8,
    PC6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM8,
    PC7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM8,
    PC8,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM8,
    PC9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn2>,
    TIM8,
    PC9,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM8,
    PC9,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::tsc::G3IO1Pin, TSC, PA15, 9u8);
pin_trait_impl!(crate::tsc::G1IO1Pin, TSC, PB12, 9u8);
pin_trait_impl!(crate::tsc::G1IO2Pin, TSC, PB13, 9u8);
pin_trait_impl!(crate::tsc::G1IO3Pin, TSC, PB14, 9u8);
pin_trait_impl!(crate::tsc::G1IO4Pin, TSC, PB15, 9u8);
pin_trait_impl!(crate::tsc::G2IO1Pin, TSC, PB4, 9u8);
pin_trait_impl!(crate::tsc::G2IO2Pin, TSC, PB5, 9u8);
pin_trait_impl!(crate::tsc::G2IO3Pin, TSC, PB6, 9u8);
pin_trait_impl!(crate::tsc::G2IO4Pin, TSC, PB7, 9u8);
pin_trait_impl!(crate::tsc::G3IO2Pin, TSC, PC10, 9u8);
pin_trait_impl!(crate::tsc::G3IO3Pin, TSC, PC11, 9u8);
pin_trait_impl!(crate::tsc::G3IO4Pin, TSC, PC12, 9u8);
pin_trait_impl!(crate::tsc::G4IO1Pin, TSC, PC6, 9u8);
pin_trait_impl!(crate::tsc::G4IO2Pin, TSC, PC7, 9u8);
pin_trait_impl!(crate::tsc::G4IO3Pin, TSC, PC8, 9u8);
pin_trait_impl!(crate::tsc::G4IO4Pin, TSC, PC9, 9u8);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PA0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PA1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART4,
    PA15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART4,
    PA15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART4,
    PB7,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PC10,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PC11,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART5,
    PB4,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART5,
    PB4,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART5,
    PB5,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART5,
    PC12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART5,
    PD2,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PA10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART1,
    PA11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART1,
    PA12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART1,
    PA12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART1,
    PA8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PA9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART1,
    PB3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART1,
    PB3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART1,
    PB4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART1,
    PB5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PB6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PB7,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART2,
    PA0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART2,
    PA1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART2,
    PA1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART2,
    PA2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART2,
    PA3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART2,
    PA4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART3,
    PA6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PB0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PB1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PB1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PB10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PB11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PB12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART3,
    PB13,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PB14,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PB14,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PC10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PC11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PC12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PC4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PC5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PD2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PD2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_FS, PA11, 10u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_FS, PA12, 10u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH1, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH3, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH2, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH4, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH3, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH5, 0u8, {});
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM6_TRGO, 0u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM8_TRGO, 1u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM7_TRGO, 2u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM5_TRGO, 3u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM2_TRGO, 4u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM4_TRGO, 5u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, EXTI9_TRG, 6u8);
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH3, 6u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH4, 5u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH4, 3u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH5, 3u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH6, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH7, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH6, 5u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH7, 5u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH4, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH5, 3u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH2, 3u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH3, 3u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA2_CH6, 4u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA2_CH7, 4u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, DMA1_CH5, 5u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, DMA2_CH7, 3u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH1, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH2, 1u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH6, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH7, 1u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH6, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH7, 1u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH3, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH4, 1u8, {});
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC1, DMA2_CH4, 7u8, {});
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDMMC1, DMA2_CH5, 7u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH2, 1u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH3, 1u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH3, 4u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH4, 4u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH4, 1u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH5, 1u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH1, 3u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH2, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH2, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH3, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH4, 7u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH6, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH7, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH5, 7u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH5, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH3, 4u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH3, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH6, 4u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH6, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH1, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH1, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH7, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH7, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH1, 4u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH2, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH5, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH7, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH7, 4u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH2, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH3, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH3, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH6, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH1, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH4, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH5, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH7, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH1, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH2, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH2, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH4, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH5, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH3, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH4, 3u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH4, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH5, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH1, 7u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH1, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH2, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH6, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH7, 7u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH3, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH5, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH1, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH2, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH4, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH5, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH6, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH7, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH6, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH7, 2u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH2, 2u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH3, 2u8, {});
pub mod triggers {
    #[allow(non_camel_case_types)]
    pub struct EXTI9_TRG;
    #[allow(non_camel_case_types)]
    pub struct TIM2_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM4_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM5_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM6_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM7_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM8_TRGO;
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Hpre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hpre::DIV1 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hpre::DIV1 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Mcopre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Mcopre::DIV1 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => 2u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => 4u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => 8u32,
            crate::pac::rcc::vals::Mcopre::DIV16 => 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Mcopre::DIV1 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV16 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllm {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllm::DIV1 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllm::DIV3 => 3u32,
            crate::pac::rcc::vals::Pllm::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllm::DIV5 => 5u32,
            crate::pac::rcc::vals::Pllm::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllm::DIV7 => 7u32,
            crate::pac::rcc::vals::Pllm::DIV8 => 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllm::DIV1 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV3 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV5 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV7 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV8 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Plln {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plln::MUL8 => 8u32,
            crate::pac::rcc::vals::Plln::MUL9 => 9u32,
            crate::pac::rcc::vals::Plln::MUL10 => 10u32,
            crate::pac::rcc::vals::Plln::MUL11 => 11u32,
            crate::pac::rcc::vals::Plln::MUL12 => 12u32,
            crate::pac::rcc::vals::Plln::MUL13 => 13u32,
            crate::pac::rcc::vals::Plln::MUL14 => 14u32,
            crate::pac::rcc::vals::Plln::MUL15 => 15u32,
            crate::pac::rcc::vals::Plln::MUL16 => 16u32,
            crate::pac::rcc::vals::Plln::MUL17 => 17u32,
            crate::pac::rcc::vals::Plln::MUL18 => 18u32,
            crate::pac::rcc::vals::Plln::MUL19 => 19u32,
            crate::pac::rcc::vals::Plln::MUL20 => 20u32,
            crate::pac::rcc::vals::Plln::MUL21 => 21u32,
            crate::pac::rcc::vals::Plln::MUL22 => 22u32,
            crate::pac::rcc::vals::Plln::MUL23 => 23u32,
            crate::pac::rcc::vals::Plln::MUL24 => 24u32,
            crate::pac::rcc::vals::Plln::MUL25 => 25u32,
            crate::pac::rcc::vals::Plln::MUL26 => 26u32,
            crate::pac::rcc::vals::Plln::MUL27 => 27u32,
            crate::pac::rcc::vals::Plln::MUL28 => 28u32,
            crate::pac::rcc::vals::Plln::MUL29 => 29u32,
            crate::pac::rcc::vals::Plln::MUL30 => 30u32,
            crate::pac::rcc::vals::Plln::MUL31 => 31u32,
            crate::pac::rcc::vals::Plln::MUL32 => 32u32,
            crate::pac::rcc::vals::Plln::MUL33 => 33u32,
            crate::pac::rcc::vals::Plln::MUL34 => 34u32,
            crate::pac::rcc::vals::Plln::MUL35 => 35u32,
            crate::pac::rcc::vals::Plln::MUL36 => 36u32,
            crate::pac::rcc::vals::Plln::MUL37 => 37u32,
            crate::pac::rcc::vals::Plln::MUL38 => 38u32,
            crate::pac::rcc::vals::Plln::MUL39 => 39u32,
            crate::pac::rcc::vals::Plln::MUL40 => 40u32,
            crate::pac::rcc::vals::Plln::MUL41 => 41u32,
            crate::pac::rcc::vals::Plln::MUL42 => 42u32,
            crate::pac::rcc::vals::Plln::MUL43 => 43u32,
            crate::pac::rcc::vals::Plln::MUL44 => 44u32,
            crate::pac::rcc::vals::Plln::MUL45 => 45u32,
            crate::pac::rcc::vals::Plln::MUL46 => 46u32,
            crate::pac::rcc::vals::Plln::MUL47 => 47u32,
            crate::pac::rcc::vals::Plln::MUL48 => 48u32,
            crate::pac::rcc::vals::Plln::MUL49 => 49u32,
            crate::pac::rcc::vals::Plln::MUL50 => 50u32,
            crate::pac::rcc::vals::Plln::MUL51 => 51u32,
            crate::pac::rcc::vals::Plln::MUL52 => 52u32,
            crate::pac::rcc::vals::Plln::MUL53 => 53u32,
            crate::pac::rcc::vals::Plln::MUL54 => 54u32,
            crate::pac::rcc::vals::Plln::MUL55 => 55u32,
            crate::pac::rcc::vals::Plln::MUL56 => 56u32,
            crate::pac::rcc::vals::Plln::MUL57 => 57u32,
            crate::pac::rcc::vals::Plln::MUL58 => 58u32,
            crate::pac::rcc::vals::Plln::MUL59 => 59u32,
            crate::pac::rcc::vals::Plln::MUL60 => 60u32,
            crate::pac::rcc::vals::Plln::MUL61 => 61u32,
            crate::pac::rcc::vals::Plln::MUL62 => 62u32,
            crate::pac::rcc::vals::Plln::MUL63 => 63u32,
            crate::pac::rcc::vals::Plln::MUL64 => 64u32,
            crate::pac::rcc::vals::Plln::MUL65 => 65u32,
            crate::pac::rcc::vals::Plln::MUL66 => 66u32,
            crate::pac::rcc::vals::Plln::MUL67 => 67u32,
            crate::pac::rcc::vals::Plln::MUL68 => 68u32,
            crate::pac::rcc::vals::Plln::MUL69 => 69u32,
            crate::pac::rcc::vals::Plln::MUL70 => 70u32,
            crate::pac::rcc::vals::Plln::MUL71 => 71u32,
            crate::pac::rcc::vals::Plln::MUL72 => 72u32,
            crate::pac::rcc::vals::Plln::MUL73 => 73u32,
            crate::pac::rcc::vals::Plln::MUL74 => 74u32,
            crate::pac::rcc::vals::Plln::MUL75 => 75u32,
            crate::pac::rcc::vals::Plln::MUL76 => 76u32,
            crate::pac::rcc::vals::Plln::MUL77 => 77u32,
            crate::pac::rcc::vals::Plln::MUL78 => 78u32,
            crate::pac::rcc::vals::Plln::MUL79 => 79u32,
            crate::pac::rcc::vals::Plln::MUL80 => 80u32,
            crate::pac::rcc::vals::Plln::MUL81 => 81u32,
            crate::pac::rcc::vals::Plln::MUL82 => 82u32,
            crate::pac::rcc::vals::Plln::MUL83 => 83u32,
            crate::pac::rcc::vals::Plln::MUL84 => 84u32,
            crate::pac::rcc::vals::Plln::MUL85 => 85u32,
            crate::pac::rcc::vals::Plln::MUL86 => 86u32,
            crate::pac::rcc::vals::Plln::MUL87 => 87u32,
            crate::pac::rcc::vals::Plln::MUL88 => 88u32,
            crate::pac::rcc::vals::Plln::MUL89 => 89u32,
            crate::pac::rcc::vals::Plln::MUL90 => 90u32,
            crate::pac::rcc::vals::Plln::MUL91 => 91u32,
            crate::pac::rcc::vals::Plln::MUL92 => 92u32,
            crate::pac::rcc::vals::Plln::MUL93 => 93u32,
            crate::pac::rcc::vals::Plln::MUL94 => 94u32,
            crate::pac::rcc::vals::Plln::MUL95 => 95u32,
            crate::pac::rcc::vals::Plln::MUL96 => 96u32,
            crate::pac::rcc::vals::Plln::MUL97 => 97u32,
            crate::pac::rcc::vals::Plln::MUL98 => 98u32,
            crate::pac::rcc::vals::Plln::MUL99 => 99u32,
            crate::pac::rcc::vals::Plln::MUL100 => 100u32,
            crate::pac::rcc::vals::Plln::MUL101 => 101u32,
            crate::pac::rcc::vals::Plln::MUL102 => 102u32,
            crate::pac::rcc::vals::Plln::MUL103 => 103u32,
            crate::pac::rcc::vals::Plln::MUL104 => 104u32,
            crate::pac::rcc::vals::Plln::MUL105 => 105u32,
            crate::pac::rcc::vals::Plln::MUL106 => 106u32,
            crate::pac::rcc::vals::Plln::MUL107 => 107u32,
            crate::pac::rcc::vals::Plln::MUL108 => 108u32,
            crate::pac::rcc::vals::Plln::MUL109 => 109u32,
            crate::pac::rcc::vals::Plln::MUL110 => 110u32,
            crate::pac::rcc::vals::Plln::MUL111 => 111u32,
            crate::pac::rcc::vals::Plln::MUL112 => 112u32,
            crate::pac::rcc::vals::Plln::MUL113 => 113u32,
            crate::pac::rcc::vals::Plln::MUL114 => 114u32,
            crate::pac::rcc::vals::Plln::MUL115 => 115u32,
            crate::pac::rcc::vals::Plln::MUL116 => 116u32,
            crate::pac::rcc::vals::Plln::MUL117 => 117u32,
            crate::pac::rcc::vals::Plln::MUL118 => 118u32,
            crate::pac::rcc::vals::Plln::MUL119 => 119u32,
            crate::pac::rcc::vals::Plln::MUL120 => 120u32,
            crate::pac::rcc::vals::Plln::MUL121 => 121u32,
            crate::pac::rcc::vals::Plln::MUL122 => 122u32,
            crate::pac::rcc::vals::Plln::MUL123 => 123u32,
            crate::pac::rcc::vals::Plln::MUL124 => 124u32,
            crate::pac::rcc::vals::Plln::MUL125 => 125u32,
            crate::pac::rcc::vals::Plln::MUL126 => 126u32,
            crate::pac::rcc::vals::Plln::MUL127 => 127u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plln::MUL8 => 1u32,
            crate::pac::rcc::vals::Plln::MUL9 => 1u32,
            crate::pac::rcc::vals::Plln::MUL10 => 1u32,
            crate::pac::rcc::vals::Plln::MUL11 => 1u32,
            crate::pac::rcc::vals::Plln::MUL12 => 1u32,
            crate::pac::rcc::vals::Plln::MUL13 => 1u32,
            crate::pac::rcc::vals::Plln::MUL14 => 1u32,
            crate::pac::rcc::vals::Plln::MUL15 => 1u32,
            crate::pac::rcc::vals::Plln::MUL16 => 1u32,
            crate::pac::rcc::vals::Plln::MUL17 => 1u32,
            crate::pac::rcc::vals::Plln::MUL18 => 1u32,
            crate::pac::rcc::vals::Plln::MUL19 => 1u32,
            crate::pac::rcc::vals::Plln::MUL20 => 1u32,
            crate::pac::rcc::vals::Plln::MUL21 => 1u32,
            crate::pac::rcc::vals::Plln::MUL22 => 1u32,
            crate::pac::rcc::vals::Plln::MUL23 => 1u32,
            crate::pac::rcc::vals::Plln::MUL24 => 1u32,
            crate::pac::rcc::vals::Plln::MUL25 => 1u32,
            crate::pac::rcc::vals::Plln::MUL26 => 1u32,
            crate::pac::rcc::vals::Plln::MUL27 => 1u32,
            crate::pac::rcc::vals::Plln::MUL28 => 1u32,
            crate::pac::rcc::vals::Plln::MUL29 => 1u32,
            crate::pac::rcc::vals::Plln::MUL30 => 1u32,
            crate::pac::rcc::vals::Plln::MUL31 => 1u32,
            crate::pac::rcc::vals::Plln::MUL32 => 1u32,
            crate::pac::rcc::vals::Plln::MUL33 => 1u32,
            crate::pac::rcc::vals::Plln::MUL34 => 1u32,
            crate::pac::rcc::vals::Plln::MUL35 => 1u32,
            crate::pac::rcc::vals::Plln::MUL36 => 1u32,
            crate::pac::rcc::vals::Plln::MUL37 => 1u32,
            crate::pac::rcc::vals::Plln::MUL38 => 1u32,
            crate::pac::rcc::vals::Plln::MUL39 => 1u32,
            crate::pac::rcc::vals::Plln::MUL40 => 1u32,
            crate::pac::rcc::vals::Plln::MUL41 => 1u32,
            crate::pac::rcc::vals::Plln::MUL42 => 1u32,
            crate::pac::rcc::vals::Plln::MUL43 => 1u32,
            crate::pac::rcc::vals::Plln::MUL44 => 1u32,
            crate::pac::rcc::vals::Plln::MUL45 => 1u32,
            crate::pac::rcc::vals::Plln::MUL46 => 1u32,
            crate::pac::rcc::vals::Plln::MUL47 => 1u32,
            crate::pac::rcc::vals::Plln::MUL48 => 1u32,
            crate::pac::rcc::vals::Plln::MUL49 => 1u32,
            crate::pac::rcc::vals::Plln::MUL50 => 1u32,
            crate::pac::rcc::vals::Plln::MUL51 => 1u32,
            crate::pac::rcc::vals::Plln::MUL52 => 1u32,
            crate::pac::rcc::vals::Plln::MUL53 => 1u32,
            crate::pac::rcc::vals::Plln::MUL54 => 1u32,
            crate::pac::rcc::vals::Plln::MUL55 => 1u32,
            crate::pac::rcc::vals::Plln::MUL56 => 1u32,
            crate::pac::rcc::vals::Plln::MUL57 => 1u32,
            crate::pac::rcc::vals::Plln::MUL58 => 1u32,
            crate::pac::rcc::vals::Plln::MUL59 => 1u32,
            crate::pac::rcc::vals::Plln::MUL60 => 1u32,
            crate::pac::rcc::vals::Plln::MUL61 => 1u32,
            crate::pac::rcc::vals::Plln::MUL62 => 1u32,
            crate::pac::rcc::vals::Plln::MUL63 => 1u32,
            crate::pac::rcc::vals::Plln::MUL64 => 1u32,
            crate::pac::rcc::vals::Plln::MUL65 => 1u32,
            crate::pac::rcc::vals::Plln::MUL66 => 1u32,
            crate::pac::rcc::vals::Plln::MUL67 => 1u32,
            crate::pac::rcc::vals::Plln::MUL68 => 1u32,
            crate::pac::rcc::vals::Plln::MUL69 => 1u32,
            crate::pac::rcc::vals::Plln::MUL70 => 1u32,
            crate::pac::rcc::vals::Plln::MUL71 => 1u32,
            crate::pac::rcc::vals::Plln::MUL72 => 1u32,
            crate::pac::rcc::vals::Plln::MUL73 => 1u32,
            crate::pac::rcc::vals::Plln::MUL74 => 1u32,
            crate::pac::rcc::vals::Plln::MUL75 => 1u32,
            crate::pac::rcc::vals::Plln::MUL76 => 1u32,
            crate::pac::rcc::vals::Plln::MUL77 => 1u32,
            crate::pac::rcc::vals::Plln::MUL78 => 1u32,
            crate::pac::rcc::vals::Plln::MUL79 => 1u32,
            crate::pac::rcc::vals::Plln::MUL80 => 1u32,
            crate::pac::rcc::vals::Plln::MUL81 => 1u32,
            crate::pac::rcc::vals::Plln::MUL82 => 1u32,
            crate::pac::rcc::vals::Plln::MUL83 => 1u32,
            crate::pac::rcc::vals::Plln::MUL84 => 1u32,
            crate::pac::rcc::vals::Plln::MUL85 => 1u32,
            crate::pac::rcc::vals::Plln::MUL86 => 1u32,
            crate::pac::rcc::vals::Plln::MUL87 => 1u32,
            crate::pac::rcc::vals::Plln::MUL88 => 1u32,
            crate::pac::rcc::vals::Plln::MUL89 => 1u32,
            crate::pac::rcc::vals::Plln::MUL90 => 1u32,
            crate::pac::rcc::vals::Plln::MUL91 => 1u32,
            crate::pac::rcc::vals::Plln::MUL92 => 1u32,
            crate::pac::rcc::vals::Plln::MUL93 => 1u32,
            crate::pac::rcc::vals::Plln::MUL94 => 1u32,
            crate::pac::rcc::vals::Plln::MUL95 => 1u32,
            crate::pac::rcc::vals::Plln::MUL96 => 1u32,
            crate::pac::rcc::vals::Plln::MUL97 => 1u32,
            crate::pac::rcc::vals::Plln::MUL98 => 1u32,
            crate::pac::rcc::vals::Plln::MUL99 => 1u32,
            crate::pac::rcc::vals::Plln::MUL100 => 1u32,
            crate::pac::rcc::vals::Plln::MUL101 => 1u32,
            crate::pac::rcc::vals::Plln::MUL102 => 1u32,
            crate::pac::rcc::vals::Plln::MUL103 => 1u32,
            crate::pac::rcc::vals::Plln::MUL104 => 1u32,
            crate::pac::rcc::vals::Plln::MUL105 => 1u32,
            crate::pac::rcc::vals::Plln::MUL106 => 1u32,
            crate::pac::rcc::vals::Plln::MUL107 => 1u32,
            crate::pac::rcc::vals::Plln::MUL108 => 1u32,
            crate::pac::rcc::vals::Plln::MUL109 => 1u32,
            crate::pac::rcc::vals::Plln::MUL110 => 1u32,
            crate::pac::rcc::vals::Plln::MUL111 => 1u32,
            crate::pac::rcc::vals::Plln::MUL112 => 1u32,
            crate::pac::rcc::vals::Plln::MUL113 => 1u32,
            crate::pac::rcc::vals::Plln::MUL114 => 1u32,
            crate::pac::rcc::vals::Plln::MUL115 => 1u32,
            crate::pac::rcc::vals::Plln::MUL116 => 1u32,
            crate::pac::rcc::vals::Plln::MUL117 => 1u32,
            crate::pac::rcc::vals::Plln::MUL118 => 1u32,
            crate::pac::rcc::vals::Plln::MUL119 => 1u32,
            crate::pac::rcc::vals::Plln::MUL120 => 1u32,
            crate::pac::rcc::vals::Plln::MUL121 => 1u32,
            crate::pac::rcc::vals::Plln::MUL122 => 1u32,
            crate::pac::rcc::vals::Plln::MUL123 => 1u32,
            crate::pac::rcc::vals::Plln::MUL124 => 1u32,
            crate::pac::rcc::vals::Plln::MUL125 => 1u32,
            crate::pac::rcc::vals::Plln::MUL126 => 1u32,
            crate::pac::rcc::vals::Plln::MUL127 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllp {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllp::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllp::DIV3 => 3u32,
            crate::pac::rcc::vals::Pllp::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllp::DIV5 => 5u32,
            crate::pac::rcc::vals::Pllp::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllp::DIV7 => 7u32,
            crate::pac::rcc::vals::Pllp::DIV8 => 8u32,
            crate::pac::rcc::vals::Pllp::DIV9 => 9u32,
            crate::pac::rcc::vals::Pllp::DIV10 => 10u32,
            crate::pac::rcc::vals::Pllp::DIV11 => 11u32,
            crate::pac::rcc::vals::Pllp::DIV12 => 12u32,
            crate::pac::rcc::vals::Pllp::DIV13 => 13u32,
            crate::pac::rcc::vals::Pllp::DIV14 => 14u32,
            crate::pac::rcc::vals::Pllp::DIV15 => 15u32,
            crate::pac::rcc::vals::Pllp::DIV16 => 16u32,
            crate::pac::rcc::vals::Pllp::DIV17 => 17u32,
            crate::pac::rcc::vals::Pllp::DIV18 => 18u32,
            crate::pac::rcc::vals::Pllp::DIV19 => 19u32,
            crate::pac::rcc::vals::Pllp::DIV20 => 20u32,
            crate::pac::rcc::vals::Pllp::DIV21 => 21u32,
            crate::pac::rcc::vals::Pllp::DIV22 => 22u32,
            crate::pac::rcc::vals::Pllp::DIV23 => 23u32,
            crate::pac::rcc::vals::Pllp::DIV24 => 24u32,
            crate::pac::rcc::vals::Pllp::DIV25 => 25u32,
            crate::pac::rcc::vals::Pllp::DIV26 => 26u32,
            crate::pac::rcc::vals::Pllp::DIV27 => 27u32,
            crate::pac::rcc::vals::Pllp::DIV28 => 28u32,
            crate::pac::rcc::vals::Pllp::DIV29 => 29u32,
            crate::pac::rcc::vals::Pllp::DIV30 => 30u32,
            crate::pac::rcc::vals::Pllp::DIV31 => 31u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllp::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV3 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV5 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV7 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV8 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV9 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV10 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV11 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV12 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV13 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV14 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV15 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV16 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV17 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV18 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV19 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV20 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV21 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV22 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV23 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV24 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV25 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV26 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV27 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV28 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV29 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV30 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV31 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllq {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllq::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllq::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllq::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllq::DIV8 => 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllq::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV8 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllr {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllr::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllr::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllr::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllr::DIV8 => 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllr::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV8 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Ppre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Ppre::DIV1 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Ppre::DIV1 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_2;
    }
    pub mod ADC123_COMMON {}
    pub mod ADC2 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_2;
    }
    pub mod ADC3 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC3;
    }
    pub mod CAN1 {
        pub type RX0 = crate::interrupt::typelevel::CAN1_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN1_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN1_SCE;
        pub type TX = crate::interrupt::typelevel::CAN1_TX;
    }
    pub mod COMP1 {
        pub type WKUP = crate::interrupt::typelevel::COMP;
    }
    pub mod COMP2 {
        pub type WKUP = crate::interrupt::typelevel::COMP;
    }
    pub mod CRC {}
    pub mod DAC1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod DBGMCU {}
    pub mod DFSDM1 {
        pub type FLT0 = crate::interrupt::typelevel::DFSDM1_FLT0;
        pub type FLT1 = crate::interrupt::typelevel::DFSDM1_FLT1;
        pub type FLT2 = crate::interrupt::typelevel::DFSDM1_FLT2;
        pub type FLT3 = crate::interrupt::typelevel::DFSDM1_FLT3;
    }
    pub mod DMA1 {
        pub type CH1 = crate::interrupt::typelevel::DMA1_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_CHANNEL2;
        pub type CH3 = crate::interrupt::typelevel::DMA1_CHANNEL3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_CHANNEL4;
        pub type CH5 = crate::interrupt::typelevel::DMA1_CHANNEL5;
        pub type CH6 = crate::interrupt::typelevel::DMA1_CHANNEL6;
        pub type CH7 = crate::interrupt::typelevel::DMA1_CHANNEL7;
    }
    pub mod DMA2 {
        pub type CH1 = crate::interrupt::typelevel::DMA2_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::DMA2_CHANNEL2;
        pub type CH3 = crate::interrupt::typelevel::DMA2_CHANNEL3;
        pub type CH4 = crate::interrupt::typelevel::DMA2_CHANNEL4;
        pub type CH5 = crate::interrupt::typelevel::DMA2_CHANNEL5;
        pub type CH6 = crate::interrupt::typelevel::DMA2_CHANNEL6;
        pub type CH7 = crate::interrupt::typelevel::DMA2_CHANNEL7;
    }
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI9_5;
    }
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOE {}
    pub mod GPIOF {}
    pub mod GPIOG {}
    pub mod GPIOH {}
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1_ER;
        pub type EV = crate::interrupt::typelevel::I2C1_EV;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2_ER;
        pub type EV = crate::interrupt::typelevel::I2C2_EV;
    }
    pub mod I2C3 {
        pub type ER = crate::interrupt::typelevel::I2C3_ER;
        pub type EV = crate::interrupt::typelevel::I2C3_EV;
    }
    pub mod IWDG {}
    pub mod LCD {
        pub type GLOBAL = crate::interrupt::typelevel::LCD;
    }
    pub mod LPTIM1 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM1;
    }
    pub mod LPTIM2 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM2;
    }
    pub mod LPUART1 {
        pub type GLOBAL = crate::interrupt::typelevel::LPUART1;
    }
    pub mod OPAMP1 {}
    pub mod OPAMP2 {}
    pub mod PWR {}
    pub mod QUADSPI {
        pub type GLOBAL = crate::interrupt::typelevel::QUADSPI;
    }
    pub mod RCC {
        pub type GLOBAL = crate::interrupt::typelevel::RCC;
    }
    pub mod RNG {
        pub type GLOBAL = crate::interrupt::typelevel::RNG;
    }
    pub mod RTC {
        pub type ALARM = crate::interrupt::typelevel::RTC_ALARM;
        pub type STAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type TAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type WKUP = crate::interrupt::typelevel::RTC_WKUP;
    }
    pub mod SAI1 {
        pub type A = crate::interrupt::typelevel::SAI1;
        pub type B = crate::interrupt::typelevel::SAI1;
    }
    pub mod SAI2 {
        pub type A = crate::interrupt::typelevel::SAI2;
        pub type B = crate::interrupt::typelevel::SAI2;
    }
    pub mod SDMMC1 {
        pub type GLOBAL = crate::interrupt::typelevel::SDMMC1;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod SPI3 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI3;
    }
    pub mod SWPMI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SWPMI1;
    }
    pub mod SYSCFG {}
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM16;
    }
    pub mod TIM15 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type CC = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type COM = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type TRG = crate::interrupt::typelevel::TIM1_BRK_TIM15;
        pub type UP = crate::interrupt::typelevel::TIM1_BRK_TIM15;
    }
    pub mod TIM16 {
        pub type BRK = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type CC = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type COM = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type TRG = crate::interrupt::typelevel::TIM1_UP_TIM16;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM16;
    }
    pub mod TIM17 {
        pub type BRK = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type CC = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
        pub type UP = crate::interrupt::typelevel::TIM1_TRG_COM_TIM17;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3;
        pub type CC = crate::interrupt::typelevel::TIM3;
        pub type COM = crate::interrupt::typelevel::TIM3;
        pub type TRG = crate::interrupt::typelevel::TIM3;
        pub type UP = crate::interrupt::typelevel::TIM3;
    }
    pub mod TIM4 {
        pub type BRK = crate::interrupt::typelevel::TIM4;
        pub type CC = crate::interrupt::typelevel::TIM4;
        pub type COM = crate::interrupt::typelevel::TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM4;
        pub type UP = crate::interrupt::typelevel::TIM4;
    }
    pub mod TIM5 {
        pub type BRK = crate::interrupt::typelevel::TIM5;
        pub type CC = crate::interrupt::typelevel::TIM5;
        pub type COM = crate::interrupt::typelevel::TIM5;
        pub type TRG = crate::interrupt::typelevel::TIM5;
        pub type UP = crate::interrupt::typelevel::TIM5;
    }
    pub mod TIM6 {
        pub type BRK = crate::interrupt::typelevel::TIM6_DAC;
        pub type CC = crate::interrupt::typelevel::TIM6_DAC;
        pub type COM = crate::interrupt::typelevel::TIM6_DAC;
        pub type TRG = crate::interrupt::typelevel::TIM6_DAC;
        pub type UP = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod TIM7 {
        pub type BRK = crate::interrupt::typelevel::TIM7;
        pub type CC = crate::interrupt::typelevel::TIM7;
        pub type COM = crate::interrupt::typelevel::TIM7;
        pub type TRG = crate::interrupt::typelevel::TIM7;
        pub type UP = crate::interrupt::typelevel::TIM7;
    }
    pub mod TIM8 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK;
        pub type CC = crate::interrupt::typelevel::TIM8_CC;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM;
        pub type UP = crate::interrupt::typelevel::TIM8_UP;
    }
    pub mod TSC {
        pub type GLOBAL = crate::interrupt::typelevel::TSC;
    }
    pub mod UART4 {
        pub type GLOBAL = crate::interrupt::typelevel::UART4;
    }
    pub mod UART5 {
        pub type GLOBAL = crate::interrupt::typelevel::UART5;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2;
    }
    pub mod USART3 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3;
    }
    pub mod USB_OTG_FS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_FS;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_FS;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_FS;
        pub type WKUP = crate::interrupt::typelevel::OTG_FS;
    }
    pub mod VREFBUF {}
    pub mod VREFINTCAL {}
    pub mod WWDG {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH1, 0u8, crate::interrupt::typelevel::DMA1_CHANNEL1);
dma_channel_impl!(DMA1_CH2, 1u8, crate::interrupt::typelevel::DMA1_CHANNEL2);
dma_channel_impl!(DMA1_CH3, 2u8, crate::interrupt::typelevel::DMA1_CHANNEL3);
dma_channel_impl!(DMA1_CH4, 3u8, crate::interrupt::typelevel::DMA1_CHANNEL4);
dma_channel_impl!(DMA1_CH5, 4u8, crate::interrupt::typelevel::DMA1_CHANNEL5);
dma_channel_impl!(DMA1_CH6, 5u8, crate::interrupt::typelevel::DMA1_CHANNEL6);
dma_channel_impl!(DMA1_CH7, 6u8, crate::interrupt::typelevel::DMA1_CHANNEL7);
dma_channel_impl!(DMA2_CH1, 7u8, crate::interrupt::typelevel::DMA2_CHANNEL1);
dma_channel_impl!(DMA2_CH2, 8u8, crate::interrupt::typelevel::DMA2_CHANNEL2);
dma_channel_impl!(DMA2_CH3, 9u8, crate::interrupt::typelevel::DMA2_CHANNEL3);
dma_channel_impl!(DMA2_CH4, 10u8, crate::interrupt::typelevel::DMA2_CHANNEL4);
dma_channel_impl!(DMA2_CH5, 11u8, crate::interrupt::typelevel::DMA2_CHANNEL5);
dma_channel_impl!(DMA2_CH6, 12u8, crate::interrupt::typelevel::DMA2_CHANNEL6);
dma_channel_impl!(DMA2_CH7, 13u8, crate::interrupt::typelevel::DMA2_CHANNEL7);
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
];
pub const fn gpio_block(port_num: usize) -> crate::pac::gpio::Gpio {
    #[cfg(stm32n6)]
    let port_num = if port_num > 7 { port_num + 5 } else { port_num };
    unsafe { crate::pac::gpio::Gpio::from_ptr((1207959552usize + 1024usize * port_num) as _) }
}
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 1048576usize;
pub const WRITE_SIZE: usize = 8usize;
