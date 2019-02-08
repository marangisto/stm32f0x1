#![no_std]
use volatile_register::{RO, WO, RW};

#[repr(C)]
/// cyclic redundancy check calculation unit
pub struct CRC {
  /// [0]: Data register
  pub dr: RW<u32>,
  /// [4]: Independent data register
  pub idr: RW<u32>,
  /// [8]: Control register
  pub cr: RW<u32>,
  /// [12]: Initial CRC value
  pub init: RW<u32>,
}

pub fn crc() -> *mut CRC {
  1073885184 as *mut CRC
}

#[repr(C)]
/// General-purpose I/Os
pub struct GPIOF {
  /// [0]: GPIO port mode register
  pub moder: RW<u32>,
  /// [4]: GPIO port output type register
  pub otyper: RW<u32>,
  /// [8]: GPIO port output speed register
  pub ospeedr: RW<u32>,
  /// [12]: GPIO port pull-up/pull-down register
  pub pupdr: RW<u32>,
  /// [16]: GPIO port input data register
  pub idr: RO<u32>,
  /// [20]: GPIO port output data register
  pub odr: RW<u32>,
  /// [24]: GPIO port bit set/reset register
  pub bsrr: WO<u32>,
  /// [28]: GPIO port configuration lock register
  pub lckr: RW<u32>,
  /// [32]: GPIO alternate function low register
  pub afrl: RW<u32>,
  /// [36]: GPIO alternate function high register
  pub afrh: RW<u32>,
  /// [40]: Port bit reset register
  pub brr: WO<u32>,
}

pub fn gpiof() -> *mut GPIOF {
  1207964672 as *mut GPIOF
}

#[repr(C)]
/// 
pub struct GPIOD { }

pub fn gpiod() -> *mut GPIOD {
  1207962624 as *mut GPIOD
}

#[repr(C)]
/// 
#[repr(C)]
/// General-purpose I/Os
pub struct GPIOC {
  /// [0]: GPIO port mode register
  pub moder: RW<u32>,
  /// [4]: GPIO port output type register
  pub otyper: RW<u32>,
  /// [8]: GPIO port output speed register
  pub ospeedr: RW<u32>,
  /// [12]: GPIO port pull-up/pull-down register
  pub pupdr: RW<u32>,
  /// [16]: GPIO port input data register
  pub idr: RO<u32>,
  /// [20]: GPIO port output data register
  pub odr: RW<u32>,
  /// [24]: GPIO port bit set/reset register
  pub bsrr: WO<u32>,
  /// [28]: GPIO port configuration lock register
  pub lckr: RW<u32>,
  /// [32]: GPIO alternate function low register
  pub afrl: RW<u32>,
  /// [36]: GPIO alternate function high register
  pub afrh: RW<u32>,
  /// [40]: Port bit reset register
  pub brr: WO<u32>,
}


pub fn gpioc() -> *mut GPIOC {
  1207961600 as *mut GPIOC
}

#[repr(C)]
/// 
pub struct GPIOB { }

pub fn gpiob() -> *mut GPIOB {
  1207960576 as *mut GPIOB
}

#[repr(C)]
/// 
pub struct GPIOE { }

pub fn gpioe() -> *mut GPIOE {
  1207963648 as *mut GPIOE
}

#[repr(C)]
/// General-purpose I/Os
pub struct GPIOA {
  /// [0]: GPIO port mode register
  pub moder: RW<u32>,
  /// [4]: GPIO port output type register
  pub otyper: RW<u32>,
  /// [8]: GPIO port output speed register
  pub ospeedr: RW<u32>,
  /// [12]: GPIO port pull-up/pull-down register
  pub pupdr: RW<u32>,
  /// [16]: GPIO port input data register
  pub idr: RO<u32>,
  /// [20]: GPIO port output data register
  pub odr: RW<u32>,
  /// [24]: GPIO port bit set/reset register
  pub bsrr: WO<u32>,
  /// [28]: GPIO port configuration lock register
  pub lckr: RW<u32>,
  /// [32]: GPIO alternate function low register
  pub afrl: RW<u32>,
  /// [36]: GPIO alternate function high register
  pub afrh: RW<u32>,
  /// [40]: Port bit reset register
  pub brr: WO<u32>,
}

pub fn gpioa() -> *mut GPIOA {
  1207959552 as *mut GPIOA
}

#[repr(C)]
/// Serial peripheral interface
pub struct SPI1 {
  /// [0]: control register 1
  pub cr1: RW<u32>,
  /// [4]: control register 2
  pub cr2: RW<u32>,
  /// [8]: status register
  pub sr: RW<u32>,
  /// [12]: data register
  pub dr: RW<u32>,
  /// [16]: CRC polynomial register
  pub crcpr: RW<u32>,
  /// [20]: RX CRC register
  pub rxcrcr: RO<u32>,
  /// [24]: TX CRC register
  pub txcrcr: RO<u32>,
  /// [28]: I2S configuration register
  pub i2scfgr: RW<u32>,
  /// [32]: I2S prescaler register
  pub i2spr: RW<u32>,
}

pub fn spi1() -> *mut SPI1 {
  1073819648 as *mut SPI1
}

#[repr(C)]
/// 
pub struct SPI2 { }

pub fn spi2() -> *mut SPI2 {
  1073756160 as *mut SPI2
}

#[repr(C)]
/// Power control
pub struct PWR {
  /// [0]: power control register
  pub cr: RW<u32>,
  /// [4]: power control/status register
  pub csr: RW<u32>,
}

pub fn pwr() -> *mut PWR {
  1073770496 as *mut PWR
}

#[repr(C)]
/// Inter-integrated circuit
pub struct I2C1 {
  /// [0]: Control register 1
  pub cr1: RW<u32>,
  /// [4]: Control register 2
  pub cr2: RW<u32>,
  /// [8]: Own address register 1
  pub oar1: RW<u32>,
  /// [12]: Own address register 2
  pub oar2: RW<u32>,
  /// [16]: Timing register
  pub timingr: RW<u32>,
  /// [20]: Status register 1
  pub timeoutr: RW<u32>,
  /// [24]: Interrupt and Status register
  pub isr: RW<u32>,
  /// [28]: Interrupt clear register
  pub icr: WO<u32>,
  /// [32]: PEC register
  pub pecr: RO<u32>,
  /// [36]: Receive data register
  pub rxdr: RO<u32>,
  /// [40]: Transmit data register
  pub txdr: RW<u32>,
}

pub fn i2c1() -> *mut I2C1 {
  1073763328 as *mut I2C1
}

#[repr(C)]
/// 
pub struct I2C2 { }

pub fn i2c2() -> *mut I2C2 {
  1073764352 as *mut I2C2
}

#[repr(C)]
/// Independent watchdog
pub struct IWDG {
  /// [0]: Key register
  pub kr: WO<u32>,
  /// [4]: Prescaler register
  pub pr: RW<u32>,
  /// [8]: Reload register
  pub rlr: RW<u32>,
  /// [12]: Status register
  pub sr: RO<u32>,
  /// [16]: Window register
  pub winr: RW<u32>,
}

pub fn iwdg() -> *mut IWDG {
  1073754112 as *mut IWDG
}

#[repr(C)]
/// Window watchdog
pub struct WWDG {
  /// [0]: Control register
  pub cr: RW<u32>,
  /// [4]: Configuration register
  pub cfr: RW<u32>,
  /// [8]: Status register
  pub sr: RW<u32>,
}

pub fn wwdg() -> *mut WWDG {
  1073753088 as *mut WWDG
}

#[repr(C)]
/// Advanced-timers
pub struct TIM1 {
  /// [0]: control register 1
  pub cr1: RW<u32>,
  /// [4]: control register 2
  pub cr2: RW<u32>,
  /// [8]: slave mode control register
  pub smcr: RW<u32>,
  /// [12]: DMA/Interrupt enable register
  pub dier: RW<u32>,
  /// [16]: status register
  pub sr: RW<u32>,
  /// [20]: event generation register
  pub egr: WO<u32>,
  /// [24]: capture/compare mode register 1 (input mode)
  pub ccmr1_input: RW<u32>,
  /// [28]: capture/compare mode register 2 (input mode)
  pub ccmr2_input: RW<u32>,
  /// [32]: capture/compare enable register
  pub ccer: RW<u32>,
  /// [36]: counter
  pub cnt: RW<u32>,
  /// [40]: prescaler
  pub psc: RW<u32>,
  /// [44]: auto-reload register
  pub arr: RW<u32>,
  /// [48]: repetition counter register
  pub rcr: RW<u32>,
  /// [52]: capture/compare register 1
  pub ccr1: RW<u32>,
  /// [56]: capture/compare register 2
  pub ccr2: RW<u32>,
  /// [60]: capture/compare register 3
  pub ccr3: RW<u32>,
  /// [64]: capture/compare register 4
  pub ccr4: RW<u32>,
  /// [68]: break and dead-time register
  pub bdtr: RW<u32>,
  /// [72]: DMA control register
  pub dcr: RW<u32>,
  /// [76]: DMA address for full transfer
  pub dmar: RW<u32>,
}

pub fn tim1() -> *mut TIM1 {
  1073818624 as *mut TIM1
}

#[repr(C)]
/// General-purpose-timers
pub struct TIM2 {
  /// [0]: control register 1
  pub cr1: RW<u32>,
  /// [4]: control register 2
  pub cr2: RW<u32>,
  /// [8]: slave mode control register
  pub smcr: RW<u32>,
  /// [12]: DMA/Interrupt enable register
  pub dier: RW<u32>,
  /// [16]: status register
  pub sr: RW<u32>,
  /// [20]: event generation register
  pub egr: WO<u32>,
  /// [24]: capture/compare mode register 1 (input mode)
  pub ccmr1_input: RW<u32>,
  /// [28]: capture/compare mode register 2 (input mode)
  pub ccmr2_input: RW<u32>,
  /// [32]: capture/compare enable register
  pub ccer: RW<u32>,
  /// [36]: counter
  pub cnt: RW<u32>,
  /// [40]: prescaler
  pub psc: RW<u32>,
  /// [44]: auto-reload register
  pub arr: RW<u32>,
  reserved0x30: u32,
  /// [52]: capture/compare register 1
  pub ccr1: RW<u32>,
  /// [56]: capture/compare register 2
  pub ccr2: RW<u32>,
  /// [60]: capture/compare register 3
  pub ccr3: RW<u32>,
  /// [64]: capture/compare register 4
  pub ccr4: RW<u32>,
  reserved0x44: u32,
  /// [72]: DMA control register
  pub dcr: RW<u32>,
  /// [76]: DMA address for full transfer
  pub dmar: RW<u32>,
}

pub fn tim2() -> *mut TIM2 {
  1073741824 as *mut TIM2
}

#[repr(C)]
/// 
pub struct TIM3 { }

pub fn tim3() -> *mut TIM3 {
  1073742848 as *mut TIM3
}

#[repr(C)]
/// General-purpose-timers
pub struct TIM14 {
  /// [0]: control register 1
  pub cr1: RW<u32>,
  reserved0x4: u32,
  reserved0x8: u32,
  /// [12]: DMA/Interrupt enable register
  pub dier: RW<u32>,
  /// [16]: status register
  pub sr: RW<u32>,
  /// [20]: event generation register
  pub egr: WO<u32>,
  /// [24]: capture/compare mode register (input mode)
  pub ccmr1_input: RW<u32>,
  reserved0x1c: u32,
  /// [32]: capture/compare enable register
  pub ccer: RW<u32>,
  /// [36]: counter
  pub cnt: RW<u32>,
  /// [40]: prescaler
  pub psc: RW<u32>,
  /// [44]: auto-reload register
  pub arr: RW<u32>,
  reserved0x30: u32,
  /// [52]: capture/compare register 1
  pub ccr1: RW<u32>,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  /// [80]: option register
  pub or: RW<u32>,
}

pub fn tim14() -> *mut TIM14 {
  1073750016 as *mut TIM14
}

#[repr(C)]
/// Basic-timers
pub struct TIM6 {
  /// [0]: control register 1
  pub cr1: RW<u32>,
  /// [4]: control register 2
  pub cr2: RW<u32>,
  reserved0x8: u32,
  /// [12]: DMA/Interrupt enable register
  pub dier: RW<u32>,
  /// [16]: status register
  pub sr: RW<u32>,
  /// [20]: event generation register
  pub egr: WO<u32>,
  reserved0x18: u32,
  reserved0x1c: u32,
  reserved0x20: u32,
  /// [36]: counter
  pub cnt: RW<u32>,
  /// [40]: prescaler
  pub psc: RW<u32>,
  /// [44]: auto-reload register
  pub arr: RW<u32>,
}

pub fn tim6() -> *mut TIM6 {
  1073745920 as *mut TIM6
}

#[repr(C)]
/// 
pub struct TIM7 { }

pub fn tim7() -> *mut TIM7 {
  1073746944 as *mut TIM7
}

#[repr(C)]
/// External interrupt/event controller
pub struct EXTI {
  /// [0]: Interrupt mask register (EXTI_IMR)
  pub imr: RW<u32>,
  /// [4]: Event mask register (EXTI_EMR)
  pub emr: RW<u32>,
  /// [8]: Rising Trigger selection register (EXTI_RTSR)
  pub rtsr: RW<u32>,
  /// [12]: Falling Trigger selection register (EXTI_FTSR)
  pub ftsr: RW<u32>,
  /// [16]: Software interrupt event register (EXTI_SWIER)
  pub swier: RW<u32>,
  /// [20]: Pending register (EXTI_PR)
  pub pr: RW<u32>,
}

pub fn exti() -> *mut EXTI {
  1073808384 as *mut EXTI
}

#[repr(C)]
/// Nested Vectored Interrupt Controller
pub struct NVIC {
  /// [0]: Interrupt Set Enable Register
  pub iser: RW<u32>,
  reserved0x4: u32,
  reserved0x8: u32,
  reserved0xc: u32,
  reserved0x10: u32,
  reserved0x14: u32,
  reserved0x18: u32,
  reserved0x1c: u32,
  reserved0x20: u32,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  /// [128]: Interrupt Clear Enable Register
  pub icer: RW<u32>,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  /// [256]: Interrupt Set-Pending Register
  pub ispr: RW<u32>,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  reserved0x114: u32,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  /// [384]: Interrupt Clear-Pending Register
  pub icpr: RW<u32>,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  reserved0x1f0: u32,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  reserved0x200: u32,
  reserved0x204: u32,
  reserved0x208: u32,
  reserved0x20c: u32,
  reserved0x210: u32,
  reserved0x214: u32,
  reserved0x218: u32,
  reserved0x21c: u32,
  reserved0x220: u32,
  reserved0x224: u32,
  reserved0x228: u32,
  reserved0x22c: u32,
  reserved0x230: u32,
  reserved0x234: u32,
  reserved0x238: u32,
  reserved0x23c: u32,
  reserved0x240: u32,
  reserved0x244: u32,
  reserved0x248: u32,
  reserved0x24c: u32,
  reserved0x250: u32,
  reserved0x254: u32,
  reserved0x258: u32,
  reserved0x25c: u32,
  reserved0x260: u32,
  reserved0x264: u32,
  reserved0x268: u32,
  reserved0x26c: u32,
  reserved0x270: u32,
  reserved0x274: u32,
  reserved0x278: u32,
  reserved0x27c: u32,
  reserved0x280: u32,
  reserved0x284: u32,
  reserved0x288: u32,
  reserved0x28c: u32,
  reserved0x290: u32,
  reserved0x294: u32,
  reserved0x298: u32,
  reserved0x29c: u32,
  reserved0x2a0: u32,
  reserved0x2a4: u32,
  reserved0x2a8: u32,
  reserved0x2ac: u32,
  reserved0x2b0: u32,
  reserved0x2b4: u32,
  reserved0x2b8: u32,
  reserved0x2bc: u32,
  reserved0x2c0: u32,
  reserved0x2c4: u32,
  reserved0x2c8: u32,
  reserved0x2cc: u32,
  reserved0x2d0: u32,
  reserved0x2d4: u32,
  reserved0x2d8: u32,
  reserved0x2dc: u32,
  reserved0x2e0: u32,
  reserved0x2e4: u32,
  reserved0x2e8: u32,
  reserved0x2ec: u32,
  reserved0x2f0: u32,
  reserved0x2f4: u32,
  reserved0x2f8: u32,
  reserved0x2fc: u32,
  /// [768]: Interrupt Priority Register 0
  pub ipr0: RW<u32>,
  /// [772]: Interrupt Priority Register 1
  pub ipr1: RW<u32>,
  /// [776]: Interrupt Priority Register 2
  pub ipr2: RW<u32>,
  /// [780]: Interrupt Priority Register 3
  pub ipr3: RW<u32>,
  /// [784]: Interrupt Priority Register 4
  pub ipr4: RW<u32>,
  /// [788]: Interrupt Priority Register 5
  pub ipr5: RW<u32>,
  /// [792]: Interrupt Priority Register 6
  pub ipr6: RW<u32>,
  /// [796]: Interrupt Priority Register 7
  pub ipr7: RW<u32>,
}

pub fn nvic() -> *mut NVIC {
  3758153984 as *mut NVIC
}

#[repr(C)]
/// DMA controller
pub struct DMA1 {
  /// [0]: DMA interrupt status register (DMA_ISR)
  pub isr: RO<u32>,
  /// [4]: DMA interrupt flag clear register (DMA_IFCR)
  pub ifcr: WO<u32>,
  /// [8]: DMA channel configuration register (DMA_CCR)
  pub ccr1: RW<u32>,
  /// [12]: DMA channel 1 number of data register
  pub cndtr1: RW<u32>,
  /// [16]: DMA channel 1 peripheral address register
  pub cpar1: RW<u32>,
  /// [20]: DMA channel 1 memory address register
  pub cmar1: RW<u32>,
  reserved0x18: u32,
  /// [28]: DMA channel configuration register (DMA_CCR)
  pub ccr2: RW<u32>,
  /// [32]: DMA channel 2 number of data register
  pub cndtr2: RW<u32>,
  /// [36]: DMA channel 2 peripheral address register
  pub cpar2: RW<u32>,
  /// [40]: DMA channel 2 memory address register
  pub cmar2: RW<u32>,
  reserved0x2c: u32,
  /// [48]: DMA channel configuration register (DMA_CCR)
  pub ccr3: RW<u32>,
  /// [52]: DMA channel 3 number of data register
  pub cndtr3: RW<u32>,
  /// [56]: DMA channel 3 peripheral address register
  pub cpar3: RW<u32>,
  /// [60]: DMA channel 3 memory address register
  pub cmar3: RW<u32>,
  reserved0x40: u32,
  /// [68]: DMA channel configuration register (DMA_CCR)
  pub ccr4: RW<u32>,
  /// [72]: DMA channel 4 number of data register
  pub cndtr4: RW<u32>,
  /// [76]: DMA channel 4 peripheral address register
  pub cpar4: RW<u32>,
  /// [80]: DMA channel 4 memory address register
  pub cmar4: RW<u32>,
  reserved0x54: u32,
  /// [88]: DMA channel configuration register (DMA_CCR)
  pub ccr5: RW<u32>,
  /// [92]: DMA channel 5 number of data register
  pub cndtr5: RW<u32>,
  /// [96]: DMA channel 5 peripheral address register
  pub cpar5: RW<u32>,
  /// [100]: DMA channel 5 memory address register
  pub cmar5: RW<u32>,
  reserved0x68: u32,
  /// [108]: DMA channel configuration register (DMA_CCR)
  pub ccr6: RW<u32>,
  /// [112]: DMA channel 6 number of data register
  pub cndtr6: RW<u32>,
  /// [116]: DMA channel 6 peripheral address register
  pub cpar6: RW<u32>,
  /// [120]: DMA channel 6 memory address register
  pub cmar6: RW<u32>,
  reserved0x7c: u32,
  /// [128]: DMA channel configuration register (DMA_CCR)
  pub ccr7: RW<u32>,
  /// [132]: DMA channel 7 number of data register
  pub cndtr7: RW<u32>,
  /// [136]: DMA channel 7 peripheral address register
  pub cpar7: RW<u32>,
  /// [140]: DMA channel 7 memory address register
  pub cmar7: RW<u32>,
}

pub fn dma1() -> *mut DMA1 {
  1073872896 as *mut DMA1
}

#[repr(C)]
/// 
pub struct DMA2 { }

pub fn dma2() -> *mut DMA2 {
  1073873920 as *mut DMA2
}

#[repr(C)]
/// Reset and clock control
pub struct RCC {
  /// [0]: Clock control register
  pub cr: RW<u32>,
  /// [4]: Clock configuration register (RCC_CFGR)
  pub cfgr: RW<u32>,
  /// [8]: Clock interrupt register (RCC_CIR)
  pub cir: RW<u32>,
  /// [12]: APB2 peripheral reset register (RCC_APB2RSTR)
  pub apb2rstr: RW<u32>,
  /// [16]: APB1 peripheral reset register (RCC_APB1RSTR)
  pub apb1rstr: RW<u32>,
  /// [20]: AHB Peripheral Clock enable register (RCC_AHBENR)
  pub ahbenr: RW<u32>,
  /// [24]: APB2 peripheral clock enable register (RCC_APB2ENR)
  pub apb2enr: RW<u32>,
  /// [28]: APB1 peripheral clock enable register (RCC_APB1ENR)
  pub apb1enr: RW<u32>,
  /// [32]: Backup domain control register (RCC_BDCR)
  pub bdcr: RW<u32>,
  /// [36]: Control/status register (RCC_CSR)
  pub csr: RW<u32>,
  /// [40]: AHB peripheral reset register
  pub ahbrstr: RW<u32>,
  /// [44]: Clock configuration register 2
  pub cfgr2: RW<u32>,
  /// [48]: Clock configuration register 3
  pub cfgr3: RW<u32>,
  /// [52]: Clock control register 2
  pub cr2: RW<u32>,
}

pub fn rcc() -> *mut RCC {
  1073876992 as *mut RCC
}

#[repr(C)]
/// System configuration controller
pub struct SYSCFG_COMP {
  /// [0]: configuration register 1
  pub syscfg_cfgr1: RW<u32>,
  reserved0x4: u32,
  /// [8]: external interrupt configuration register 1
  pub syscfg_exticr1: RW<u32>,
  /// [12]: external interrupt configuration register 2
  pub syscfg_exticr2: RW<u32>,
  /// [16]: external interrupt configuration register 3
  pub syscfg_exticr3: RW<u32>,
  /// [20]: external interrupt configuration register 4
  pub syscfg_exticr4: RW<u32>,
  /// [24]: configuration register 2
  pub syscfg_cfgr2: RW<u32>,
  /// [28]: control and status register
  pub comp_csr: RW<u32>,
}

pub fn syscfg_comp() -> *mut SYSCFG_COMP {
  1073807360 as *mut SYSCFG_COMP
}

#[repr(C)]
/// Analog-to-digital converter
pub struct ADC {
  /// [0]: interrupt and status register
  pub isr: RW<u32>,
  /// [4]: interrupt enable register
  pub ier: RW<u32>,
  /// [8]: control register
  pub cr: RW<u32>,
  /// [12]: configuration register 1
  pub cfgr1: RW<u32>,
  /// [16]: configuration register 2
  pub cfgr2: RW<u32>,
  /// [20]: sampling time register
  pub smpr: RW<u32>,
  reserved0x18: u32,
  reserved0x1c: u32,
  /// [32]: watchdog threshold register
  pub tr: RW<u32>,
  reserved0x24: u32,
  /// [40]: channel selection register
  pub chselr: RW<u32>,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: data register
  pub dr: RO<u32>,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  reserved0x114: u32,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  reserved0x180: u32,
  reserved0x184: u32,
  reserved0x188: u32,
  reserved0x18c: u32,
  reserved0x190: u32,
  reserved0x194: u32,
  reserved0x198: u32,
  reserved0x19c: u32,
  reserved0x1a0: u32,
  reserved0x1a4: u32,
  reserved0x1a8: u32,
  reserved0x1ac: u32,
  reserved0x1b0: u32,
  reserved0x1b4: u32,
  reserved0x1b8: u32,
  reserved0x1bc: u32,
  reserved0x1c0: u32,
  reserved0x1c4: u32,
  reserved0x1c8: u32,
  reserved0x1cc: u32,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  reserved0x1f0: u32,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  reserved0x200: u32,
  reserved0x204: u32,
  reserved0x208: u32,
  reserved0x20c: u32,
  reserved0x210: u32,
  reserved0x214: u32,
  reserved0x218: u32,
  reserved0x21c: u32,
  reserved0x220: u32,
  reserved0x224: u32,
  reserved0x228: u32,
  reserved0x22c: u32,
  reserved0x230: u32,
  reserved0x234: u32,
  reserved0x238: u32,
  reserved0x23c: u32,
  reserved0x240: u32,
  reserved0x244: u32,
  reserved0x248: u32,
  reserved0x24c: u32,
  reserved0x250: u32,
  reserved0x254: u32,
  reserved0x258: u32,
  reserved0x25c: u32,
  reserved0x260: u32,
  reserved0x264: u32,
  reserved0x268: u32,
  reserved0x26c: u32,
  reserved0x270: u32,
  reserved0x274: u32,
  reserved0x278: u32,
  reserved0x27c: u32,
  reserved0x280: u32,
  reserved0x284: u32,
  reserved0x288: u32,
  reserved0x28c: u32,
  reserved0x290: u32,
  reserved0x294: u32,
  reserved0x298: u32,
  reserved0x29c: u32,
  reserved0x2a0: u32,
  reserved0x2a4: u32,
  reserved0x2a8: u32,
  reserved0x2ac: u32,
  reserved0x2b0: u32,
  reserved0x2b4: u32,
  reserved0x2b8: u32,
  reserved0x2bc: u32,
  reserved0x2c0: u32,
  reserved0x2c4: u32,
  reserved0x2c8: u32,
  reserved0x2cc: u32,
  reserved0x2d0: u32,
  reserved0x2d4: u32,
  reserved0x2d8: u32,
  reserved0x2dc: u32,
  reserved0x2e0: u32,
  reserved0x2e4: u32,
  reserved0x2e8: u32,
  reserved0x2ec: u32,
  reserved0x2f0: u32,
  reserved0x2f4: u32,
  reserved0x2f8: u32,
  reserved0x2fc: u32,
  reserved0x300: u32,
  reserved0x304: u32,
  /// [776]: common configuration register
  pub ccr: RW<u32>,
}

pub fn adc() -> *mut ADC {
  1073816576 as *mut ADC
}

#[repr(C)]
/// Universal synchronous asynchronous receiver transmitter
pub struct USART1 {
  /// [0]: Control register 1
  pub cr1: RW<u32>,
  /// [4]: Control register 2
  pub cr2: RW<u32>,
  /// [8]: Control register 3
  pub cr3: RW<u32>,
  /// [12]: Baud rate register
  pub brr: RW<u32>,
  /// [16]: Guard time and prescaler register
  pub gtpr: RW<u32>,
  /// [20]: Receiver timeout register
  pub rtor: RW<u32>,
  /// [24]: Request register
  pub rqr: RW<u32>,
  /// [28]: Interrupt &amp; status register
  pub isr: RO<u32>,
  /// [32]: Interrupt flag clear register
  pub icr: RW<u32>,
  /// [36]: Receive data register
  pub rdr: RO<u32>,
  /// [40]: Transmit data register
  pub tdr: RW<u32>,
}

pub fn usart1() -> *mut USART1 {
  1073821696 as *mut USART1
}

#[repr(C)]
/// 
pub struct USART2 { }

pub fn usart2() -> *mut USART2 {
  1073759232 as *mut USART2
}

#[repr(C)]
/// 
pub struct USART3 { }

pub fn usart3() -> *mut USART3 {
  1073760256 as *mut USART3
}

#[repr(C)]
/// 
pub struct USART4 { }

pub fn usart4() -> *mut USART4 {
  1073761280 as *mut USART4
}

#[repr(C)]
/// 
pub struct USART6 { }

pub fn usart6() -> *mut USART6 {
  1073812480 as *mut USART6
}

#[repr(C)]
/// 
pub struct USART7 { }

pub fn usart7() -> *mut USART7 {
  1073813504 as *mut USART7
}

#[repr(C)]
/// 
pub struct USART8 { }

pub fn usart8() -> *mut USART8 {
  1073814528 as *mut USART8
}

#[repr(C)]
/// 
pub struct USART5 { }

pub fn usart5() -> *mut USART5 {
  1073762304 as *mut USART5
}

#[repr(C)]
/// Real-time clock
pub struct RTC {
  /// [0]: time register
  pub tr: RW<u32>,
  /// [4]: date register
  pub dr: RW<u32>,
  /// [8]: control register
  pub cr: RW<u32>,
  /// [12]: initialization and status register
  pub isr: RW<u32>,
  /// [16]: prescaler register
  pub prer: RW<u32>,
  reserved0x14: u32,
  reserved0x18: u32,
  /// [28]: alarm A register
  pub alrmar: RW<u32>,
  reserved0x20: u32,
  /// [36]: write protection register
  pub wpr: WO<u32>,
  /// [40]: sub second register
  pub ssr: RO<u32>,
  /// [44]: shift control register
  pub shiftr: WO<u32>,
  /// [48]: timestamp time register
  pub tstr: RO<u32>,
  /// [52]: timestamp date register
  pub tsdr: RO<u32>,
  /// [56]: time-stamp sub second register
  pub tsssr: RO<u32>,
  /// [60]: calibration register
  pub calr: RW<u32>,
  /// [64]: tamper and alternate function configuration register
  pub tafcr: RW<u32>,
  /// [68]: alarm A sub second register
  pub alrmassr: RW<u32>,
  reserved0x48: u32,
  reserved0x4c: u32,
  /// [80]: backup register
  pub bkp0r: RW<u32>,
  /// [84]: backup register
  pub bkp1r: RW<u32>,
  /// [88]: backup register
  pub bkp2r: RW<u32>,
  /// [92]: backup register
  pub bkp3r: RW<u32>,
  /// [96]: backup register
  pub bkp4r: RW<u32>,
}

pub fn rtc() -> *mut RTC {
  1073752064 as *mut RTC
}

#[repr(C)]
/// General-purpose-timers
pub struct TIM15 {
  /// [0]: control register 1
  pub cr1: RW<u32>,
  /// [4]: control register 2
  pub cr2: RW<u32>,
  /// [8]: slave mode control register
  pub smcr: RW<u32>,
  /// [12]: DMA/Interrupt enable register
  pub dier: RW<u32>,
  /// [16]: status register
  pub sr: RW<u32>,
  /// [20]: event generation register
  pub egr: WO<u32>,
  /// [24]: capture/compare mode register 1 (input mode)
  pub ccmr1_input: RW<u32>,
  reserved0x1c: u32,
  /// [32]: capture/compare enable register
  pub ccer: RW<u32>,
  /// [36]: counter
  pub cnt: RW<u32>,
  /// [40]: prescaler
  pub psc: RW<u32>,
  /// [44]: auto-reload register
  pub arr: RW<u32>,
  /// [48]: repetition counter register
  pub rcr: RW<u32>,
  /// [52]: capture/compare register 1
  pub ccr1: RW<u32>,
  /// [56]: capture/compare register 2
  pub ccr2: RW<u32>,
  reserved0x3c: u32,
  reserved0x40: u32,
  /// [68]: break and dead-time register
  pub bdtr: RW<u32>,
  /// [72]: DMA control register
  pub dcr: RW<u32>,
  /// [76]: DMA address for full transfer
  pub dmar: RW<u32>,
}

pub fn tim15() -> *mut TIM15 {
  1073823744 as *mut TIM15
}

#[repr(C)]
/// General-purpose-timers
pub struct TIM16 {
  /// [0]: control register 1
  pub cr1: RW<u32>,
  /// [4]: control register 2
  pub cr2: RW<u32>,
  reserved0x8: u32,
  /// [12]: DMA/Interrupt enable register
  pub dier: RW<u32>,
  /// [16]: status register
  pub sr: RW<u32>,
  /// [20]: event generation register
  pub egr: WO<u32>,
  /// [24]: capture/compare mode register 1 (input mode)
  pub ccmr1_input: RW<u32>,
  reserved0x1c: u32,
  /// [32]: capture/compare enable register
  pub ccer: RW<u32>,
  /// [36]: counter
  pub cnt: RW<u32>,
  /// [40]: prescaler
  pub psc: RW<u32>,
  /// [44]: auto-reload register
  pub arr: RW<u32>,
  /// [48]: repetition counter register
  pub rcr: RW<u32>,
  /// [52]: capture/compare register 1
  pub ccr1: RW<u32>,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  /// [68]: break and dead-time register
  pub bdtr: RW<u32>,
  /// [72]: DMA control register
  pub dcr: RW<u32>,
  /// [76]: DMA address for full transfer
  pub dmar: RW<u32>,
}

pub fn tim16() -> *mut TIM16 {
  1073824768 as *mut TIM16
}

#[repr(C)]
/// 
pub struct TIM17 { }

pub fn tim17() -> *mut TIM17 {
  1073825792 as *mut TIM17
}

#[repr(C)]
/// Touch sensing controller
pub struct TSC {
  /// [0]: control register
  pub cr: RW<u32>,
  /// [4]: interrupt enable register
  pub ier: RW<u32>,
  /// [8]: interrupt clear register
  pub icr: RW<u32>,
  /// [12]: interrupt status register
  pub isr: RW<u32>,
  /// [16]: I/O hysteresis control register
  pub iohcr: RW<u32>,
  reserved0x14: u32,
  /// [24]: I/O analog switch control register
  pub ioascr: RW<u32>,
  reserved0x1c: u32,
  /// [32]: I/O sampling control register
  pub ioscr: RW<u32>,
  reserved0x24: u32,
  /// [40]: I/O channel control register
  pub ioccr: RW<u32>,
  reserved0x2c: u32,
  /// [48]: I/O group control status register
  pub iogcsr: RW<u32>,
  /// [52]: I/O group x counter register
  pub iog1cr: RO<u32>,
  /// [56]: I/O group x counter register
  pub iog2cr: RO<u32>,
  /// [60]: I/O group x counter register
  pub iog3cr: RO<u32>,
  /// [64]: I/O group x counter register
  pub iog4cr: RO<u32>,
  /// [68]: I/O group x counter register
  pub iog5cr: RO<u32>,
  /// [72]: I/O group x counter register
  pub iog6cr: RO<u32>,
}

pub fn tsc() -> *mut TSC {
  1073889280 as *mut TSC
}

#[repr(C)]
/// HDMI-CEC controller
pub struct CEC {
  /// [0]: control register
  pub cr: RW<u32>,
  /// [4]: configuration register
  pub cfgr: RW<u32>,
  /// [8]: Tx data register
  pub txdr: WO<u32>,
  /// [12]: Rx Data Register
  pub rxdr: RO<u32>,
  /// [16]: Interrupt and Status Register
  pub isr: RW<u32>,
  /// [20]: interrupt enable register
  pub ier: RW<u32>,
}

pub fn cec() -> *mut CEC {
  1073772544 as *mut CEC
}

#[repr(C)]
/// Flash
pub struct Flash {
  /// [0]: Flash access control register
  pub acr: RW<u32>,
  /// [4]: Flash key register
  pub keyr: WO<u32>,
  /// [8]: Flash option key register
  pub optkeyr: WO<u32>,
  /// [12]: Flash status register
  pub sr: RW<u32>,
  /// [16]: Flash control register
  pub cr: RW<u32>,
  /// [20]: Flash address register
  pub ar: WO<u32>,
  reserved0x18: u32,
  /// [28]: Option byte register
  pub obr: RO<u32>,
  /// [32]: Write protection register
  pub wrpr: RO<u32>,
}

pub fn flash() -> *mut Flash {
  1073881088 as *mut Flash
}

#[repr(C)]
/// Debug support
pub struct DBGMCU {
  /// [0]: MCU Device ID Code Register
  pub idcode: RO<u32>,
  /// [4]: Debug MCU Configuration Register
  pub cr: RW<u32>,
  /// [8]: Debug MCU APB1 freeze register
  pub apb1_fz: RW<u32>,
  /// [12]: Debug MCU APB2 freeze register
  pub apb2_fz: RW<u32>,
}

pub fn dbgmcu() -> *mut DBGMCU {
  1073829888 as *mut DBGMCU
}

#[repr(C)]
/// Universal serial bus full-speed device interface
pub struct USB {
  /// [0]: endpoint 0 register
  pub ep0r: RW<u32>,
  /// [4]: endpoint 1 register
  pub ep1r: RW<u32>,
  /// [8]: endpoint 2 register
  pub ep2r: RW<u32>,
  /// [12]: endpoint 3 register
  pub ep3r: RW<u32>,
  /// [16]: endpoint 4 register
  pub ep4r: RW<u32>,
  /// [20]: endpoint 5 register
  pub ep5r: RW<u32>,
  /// [24]: endpoint 6 register
  pub ep6r: RW<u32>,
  /// [28]: endpoint 7 register
  pub ep7r: RW<u32>,
  reserved0x20: u32,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  /// [64]: control register
  pub cntr: RW<u32>,
  /// [68]: interrupt status register
  pub istr: RW<u32>,
  /// [72]: frame number register
  pub fnr: RO<u32>,
  /// [76]: device address
  pub daddr: RW<u32>,
  /// [80]: Buffer table address
  pub btable: RW<u32>,
  /// [84]: LPM control and status register
  pub lpmcsr: RW<u32>,
  /// [88]: Battery charging detector
  pub bcdr: RW<u32>,
}

pub fn usb() -> *mut USB {
  1073765376 as *mut USB
}

#[repr(C)]
/// Clock recovery system
pub struct CRS {
  /// [0]: control register
  pub cr: RW<u32>,
  /// [4]: configuration register
  pub cfgr: RW<u32>,
  /// [8]: interrupt and status register
  pub isr: RO<u32>,
  /// [12]: interrupt flag clear register
  pub icr: RW<u32>,
}

pub fn crs() -> *mut CRS {
  1073769472 as *mut CRS
}

#[repr(C)]
/// Controller area network
pub struct CAN {
  /// [0]: CAN_MCR
  pub can_mcr: RW<u32>,
  /// [4]: CAN_MSR
  pub can_msr: RW<u32>,
  /// [8]: CAN_TSR
  pub can_tsr: RW<u32>,
  /// [12]: CAN_RF0R
  pub can_rf0r: RW<u32>,
  /// [16]: CAN_RF1R
  pub can_rf1r: RW<u32>,
  /// [20]: CAN_IER
  pub can_ier: RW<u32>,
  /// [24]: CAN_ESR
  pub can_esr: RW<u32>,
  /// [28]: CAN BTR
  pub can_btr: RW<u32>,
  reserved0x20: u32,
  reserved0x24: u32,
  reserved0x28: u32,
  reserved0x2c: u32,
  reserved0x30: u32,
  reserved0x34: u32,
  reserved0x38: u32,
  reserved0x3c: u32,
  reserved0x40: u32,
  reserved0x44: u32,
  reserved0x48: u32,
  reserved0x4c: u32,
  reserved0x50: u32,
  reserved0x54: u32,
  reserved0x58: u32,
  reserved0x5c: u32,
  reserved0x60: u32,
  reserved0x64: u32,
  reserved0x68: u32,
  reserved0x6c: u32,
  reserved0x70: u32,
  reserved0x74: u32,
  reserved0x78: u32,
  reserved0x7c: u32,
  reserved0x80: u32,
  reserved0x84: u32,
  reserved0x88: u32,
  reserved0x8c: u32,
  reserved0x90: u32,
  reserved0x94: u32,
  reserved0x98: u32,
  reserved0x9c: u32,
  reserved0xa0: u32,
  reserved0xa4: u32,
  reserved0xa8: u32,
  reserved0xac: u32,
  reserved0xb0: u32,
  reserved0xb4: u32,
  reserved0xb8: u32,
  reserved0xbc: u32,
  reserved0xc0: u32,
  reserved0xc4: u32,
  reserved0xc8: u32,
  reserved0xcc: u32,
  reserved0xd0: u32,
  reserved0xd4: u32,
  reserved0xd8: u32,
  reserved0xdc: u32,
  reserved0xe0: u32,
  reserved0xe4: u32,
  reserved0xe8: u32,
  reserved0xec: u32,
  reserved0xf0: u32,
  reserved0xf4: u32,
  reserved0xf8: u32,
  reserved0xfc: u32,
  reserved0x100: u32,
  reserved0x104: u32,
  reserved0x108: u32,
  reserved0x10c: u32,
  reserved0x110: u32,
  reserved0x114: u32,
  reserved0x118: u32,
  reserved0x11c: u32,
  reserved0x120: u32,
  reserved0x124: u32,
  reserved0x128: u32,
  reserved0x12c: u32,
  reserved0x130: u32,
  reserved0x134: u32,
  reserved0x138: u32,
  reserved0x13c: u32,
  reserved0x140: u32,
  reserved0x144: u32,
  reserved0x148: u32,
  reserved0x14c: u32,
  reserved0x150: u32,
  reserved0x154: u32,
  reserved0x158: u32,
  reserved0x15c: u32,
  reserved0x160: u32,
  reserved0x164: u32,
  reserved0x168: u32,
  reserved0x16c: u32,
  reserved0x170: u32,
  reserved0x174: u32,
  reserved0x178: u32,
  reserved0x17c: u32,
  /// [384]: CAN_TI0R
  pub can_ti0r: RW<u32>,
  /// [388]: CAN_TDT0R
  pub can_tdt0r: RW<u32>,
  /// [392]: CAN_TDL0R
  pub can_tdl0r: RW<u32>,
  /// [396]: CAN_TDH0R
  pub can_tdh0r: RW<u32>,
  /// [400]: CAN_TI1R
  pub can_ti1r: RW<u32>,
  /// [404]: CAN_TDT1R
  pub can_tdt1r: RW<u32>,
  /// [408]: CAN_TDL1R
  pub can_tdl1r: RW<u32>,
  /// [412]: CAN_TDH1R
  pub can_tdh1r: RW<u32>,
  /// [416]: CAN_TI2R
  pub can_ti2r: RW<u32>,
  /// [420]: CAN_TDT2R
  pub can_tdt2r: RW<u32>,
  /// [424]: CAN_TDL2R
  pub can_tdl2r: RW<u32>,
  /// [428]: CAN_TDH2R
  pub can_tdh2r: RW<u32>,
  /// [432]: CAN_RI0R
  pub can_ri0r: RO<u32>,
  /// [436]: CAN_RDT0R
  pub can_rdt0r: RO<u32>,
  /// [440]: CAN_RDL0R
  pub can_rdl0r: RO<u32>,
  /// [444]: CAN_RDH0R
  pub can_rdh0r: RO<u32>,
  /// [448]: CAN_RI1R
  pub can_ri1r: RO<u32>,
  /// [452]: CAN_RDT1R
  pub can_rdt1r: RO<u32>,
  /// [456]: CAN_RDL1R
  pub can_rdl1r: RO<u32>,
  /// [460]: CAN_RDH1R
  pub can_rdh1r: RO<u32>,
  reserved0x1d0: u32,
  reserved0x1d4: u32,
  reserved0x1d8: u32,
  reserved0x1dc: u32,
  reserved0x1e0: u32,
  reserved0x1e4: u32,
  reserved0x1e8: u32,
  reserved0x1ec: u32,
  reserved0x1f0: u32,
  reserved0x1f4: u32,
  reserved0x1f8: u32,
  reserved0x1fc: u32,
  /// [512]: CAN_FMR
  pub can_fmr: RW<u32>,
  /// [516]: CAN_FM1R
  pub can_fm1r: RW<u32>,
  reserved0x208: u32,
  /// [524]: CAN_FS1R
  pub can_fs1r: RW<u32>,
  reserved0x210: u32,
  /// [532]: CAN_FFA1R
  pub can_ffa1r: RW<u32>,
  reserved0x218: u32,
  /// [540]: CAN_FA1R
  pub can_fa1r: RW<u32>,
  reserved0x220: u32,
  reserved0x224: u32,
  reserved0x228: u32,
  reserved0x22c: u32,
  reserved0x230: u32,
  reserved0x234: u32,
  reserved0x238: u32,
  reserved0x23c: u32,
  /// [576]: Filter bank 0 register 1
  pub f0r1: RW<u32>,
  /// [580]: Filter bank 0 register 2
  pub f0r2: RW<u32>,
  /// [584]: Filter bank 1 register 1
  pub f1r1: RW<u32>,
  /// [588]: Filter bank 1 register 2
  pub f1r2: RW<u32>,
  /// [592]: Filter bank 2 register 1
  pub f2r1: RW<u32>,
  /// [596]: Filter bank 2 register 2
  pub f2r2: RW<u32>,
  /// [600]: Filter bank 3 register 1
  pub f3r1: RW<u32>,
  /// [604]: Filter bank 3 register 2
  pub f3r2: RW<u32>,
  /// [608]: Filter bank 4 register 1
  pub f4r1: RW<u32>,
  /// [612]: Filter bank 4 register 2
  pub f4r2: RW<u32>,
  /// [616]: Filter bank 5 register 1
  pub f5r1: RW<u32>,
  /// [620]: Filter bank 5 register 2
  pub f5r2: RW<u32>,
  /// [624]: Filter bank 6 register 1
  pub f6r1: RW<u32>,
  /// [628]: Filter bank 6 register 2
  pub f6r2: RW<u32>,
  /// [632]: Filter bank 7 register 1
  pub f7r1: RW<u32>,
  /// [636]: Filter bank 7 register 2
  pub f7r2: RW<u32>,
  /// [640]: Filter bank 8 register 1
  pub f8r1: RW<u32>,
  /// [644]: Filter bank 8 register 2
  pub f8r2: RW<u32>,
  /// [648]: Filter bank 9 register 1
  pub f9r1: RW<u32>,
  /// [652]: Filter bank 9 register 2
  pub f9r2: RW<u32>,
  /// [656]: Filter bank 10 register 1
  pub f10r1: RW<u32>,
  /// [660]: Filter bank 10 register 2
  pub f10r2: RW<u32>,
  /// [664]: Filter bank 11 register 1
  pub f11r1: RW<u32>,
  /// [668]: Filter bank 11 register 2
  pub f11r2: RW<u32>,
  /// [672]: Filter bank 4 register 1
  pub f12r1: RW<u32>,
  /// [676]: Filter bank 12 register 2
  pub f12r2: RW<u32>,
  /// [680]: Filter bank 13 register 1
  pub f13r1: RW<u32>,
  /// [684]: Filter bank 13 register 2
  pub f13r2: RW<u32>,
  /// [688]: Filter bank 14 register 1
  pub f14r1: RW<u32>,
  /// [692]: Filter bank 14 register 2
  pub f14r2: RW<u32>,
  /// [696]: Filter bank 15 register 1
  pub f15r1: RW<u32>,
  /// [700]: Filter bank 15 register 2
  pub f15r2: RW<u32>,
  /// [704]: Filter bank 16 register 1
  pub f16r1: RW<u32>,
  /// [708]: Filter bank 16 register 2
  pub f16r2: RW<u32>,
  /// [712]: Filter bank 17 register 1
  pub f17r1: RW<u32>,
  /// [716]: Filter bank 17 register 2
  pub f17r2: RW<u32>,
  /// [720]: Filter bank 18 register 1
  pub f18r1: RW<u32>,
  /// [724]: Filter bank 18 register 2
  pub f18r2: RW<u32>,
  /// [728]: Filter bank 19 register 1
  pub f19r1: RW<u32>,
  /// [732]: Filter bank 19 register 2
  pub f19r2: RW<u32>,
  /// [736]: Filter bank 20 register 1
  pub f20r1: RW<u32>,
  /// [740]: Filter bank 20 register 2
  pub f20r2: RW<u32>,
  /// [744]: Filter bank 21 register 1
  pub f21r1: RW<u32>,
  /// [748]: Filter bank 21 register 2
  pub f21r2: RW<u32>,
  /// [752]: Filter bank 22 register 1
  pub f22r1: RW<u32>,
  /// [756]: Filter bank 22 register 2
  pub f22r2: RW<u32>,
  /// [760]: Filter bank 23 register 1
  pub f23r1: RW<u32>,
  /// [764]: Filter bank 23 register 2
  pub f23r2: RW<u32>,
  /// [768]: Filter bank 24 register 1
  pub f24r1: RW<u32>,
  /// [772]: Filter bank 24 register 2
  pub f24r2: RW<u32>,
  /// [776]: Filter bank 25 register 1
  pub f25r1: RW<u32>,
  /// [780]: Filter bank 25 register 2
  pub f25r2: RW<u32>,
  /// [784]: Filter bank 26 register 1
  pub f26r1: RW<u32>,
  /// [788]: Filter bank 26 register 2
  pub f26r2: RW<u32>,
  /// [792]: Filter bank 27 register 1
  pub f27r1: RW<u32>,
  /// [796]: Filter bank 27 register 2
  pub f27r2: RW<u32>,
}

pub fn can() -> *mut CAN {
  1073767424 as *mut CAN
}

#[repr(C)]
/// Digital-to-analog converter
pub struct DAC {
  /// [0]: control register
  pub cr: RW<u32>,
  /// [4]: software trigger register
  pub swtrigr: WO<u32>,
  /// [8]: channel1 12-bit right-aligned data holding register
  pub dhr12r1: RW<u32>,
  /// [12]: channel1 12-bit left aligned data holding register
  pub dhr12l1: RW<u32>,
  /// [16]: channel1 8-bit right aligned data holding register
  pub dhr8r1: RW<u32>,
  /// [20]: DAC channel2 12-bit right-aligned data holding register
  pub dhr12r2: RW<u32>,
  /// [24]: DAC channel2 12-bit left-aligned data holding register
  pub dhr12l2: RW<u32>,
  /// [28]: DAC channel2 8-bit right-aligned data holding register
  pub dhr8r2: RW<u32>,
  /// [32]: DHR12RD
  pub dhr12rd: RW<u32>,
  /// [36]: Dual DAC 12-bit left-aligned data holding register
  pub dhr12ld: RW<u32>,
  /// [40]: Dual DAC 8-bit right-aligned data holding register
  pub dhr8rd: RW<u32>,
  /// [44]: channel1 data output register
  pub dor1: RO<u32>,
  /// [48]: DAC channel2 data output register
  pub dor2: RO<u32>,
  /// [52]: status register
  pub sr: RW<u32>,
}

pub fn dac() -> *mut DAC {
  1073771520 as *mut DAC
}

#[repr(C)]
/// System control block
pub struct SCB {
  /// [0]: CPUID base register
  pub cpuid: RO<u32>,
  /// [4]: Interrupt control and state register
  pub icsr: RW<u32>,
  reserved0x8: u32,
  /// [12]: Application interrupt and reset control register
  pub aircr: RW<u32>,
  /// [16]: System control register
  pub scr: RW<u32>,
  /// [20]: Configuration and control register
  pub ccr: RW<u32>,
  reserved0x18: u32,
  /// [28]: System handler priority registers
  pub shpr2: RW<u32>,
  /// [32]: System handler priority registers
  pub shpr3: RW<u32>,
}

pub fn scb() -> *mut SCB {
  3758157056 as *mut SCB
}

#[repr(C)]
/// SysTick timer
pub struct STK {
  /// [0]: SysTick control and status register
  pub csr: RW<u32>,
  /// [4]: SysTick reload value register
  pub rvr: RW<u32>,
  /// [8]: SysTick current value register
  pub cvr: RW<u32>,
  /// [12]: SysTick calibration value register
  pub calib: RW<u32>,
}

pub fn stk() -> *mut STK {
  3758153744 as *mut STK
}
