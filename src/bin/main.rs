#![no_std]
#![no_main]
#![feature(naked_functions)]
#![allow(named_asm_labels)]
#![allow(undefined_naked_function_abi)]

use asm::ptr::write_slice;
use core::arch::{naked_asm};
use asm::{const_vec, vector};
use asm::vector::{Vector, VectorTable};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern {
    static LD_STACK_PTR: usize;
}

// Original Vector Table
static ISR_TABLE : VectorTable<Vector, 109> = const_vec!(109, vector!(null),
    vector!("STP", extern LD_STACK_PTR),
    // Thumb-mode, least-significant bit will be set
    vector!("Reset", fn main),
    vector!("NMI", fn irq_default),
    vector!("HardFault", fn irq_default),
    vector!("MMUFault", fn irq_default),
    vector!("BusFault", fn irq_default),
    vector!("UsageFault", fn irq_default),
    vector!(null),
    vector!(null),
    vector!(null),
    vector!(null),
    vector!("SVC", fn irq_default),
    vector!(null),
    vector!(null),
    vector!("PendSV", fn irq_default),
    vector!("Systick", fn irq_default),
    // https://www.st.com/resource/en/reference_manual/rm0090-stm32f405415-stm32f407417-stm32f427437-and-stm32f429439-advanced-armbased-32bit-mcus-stmicroelectronics.pdf
    vector!("WWDG", fn irq_default),
    vector!("PVD", fn irq_default),
    vector!("TAMP_STAMP", fn irq_default),
    vector!("RTC_WKUP", fn irq_default),
    vector!("FLASH", fn irq_default),
    vector!("RCC", fn irq_default),
    vector!("EXTI0", fn irq_default),
    vector!("EXTI1", fn irq_default),
    vector!("EXTI2", fn irq_default),
    vector!("EXTI3", fn irq_default),
    vector!("EXTI4", fn irq_default),
    vector!("DMA1_Stream0", fn irq_default),
    vector!("DMA1_Stream1", fn irq_default),
    vector!("DMA1_Stream2", fn irq_default),
    vector!("DMA1_Stream3", fn irq_default),
    vector!("DMA1_Stream4", fn irq_default),
    vector!("DMA1_Stream5", fn irq_default),
    vector!("DMA1_Stream6", fn irq_default),
    vector!("ADC", fn irq_default),
    vector!("CAN1_TX", fn irq_default),
    vector!("CAN1_RX0", fn irq_default),
    vector!("CAN1_RX1", fn irq_default),
    vector!("CAN1_SCE", fn irq_default),
    vector!("EXTI9_5", fn irq_default),
    vector!("TIM1_BRK_TIM9", fn irq_default),
    vector!("TIM1_BRK_TIM10", fn irq_default),
    vector!("TIM1_TRG_COM_TIM11", fn irq_default),
    vector!("TIM1_CC", fn irq_default),
    vector!("TIM2", fn irq_default),
    vector!("TIM3", fn irq_default),
    vector!("TIM4", fn irq_default),
    vector!("I2C1_EV", fn irq_default),
    vector!("I2C1_ER", fn irq_default),
    vector!("I2C2_EV", fn irq_default),
    vector!("I2C2_ER", fn irq_default),
    vector!("SPI1", fn irq_default),
    vector!("SPI2", fn irq_default),
    vector!("USART1", fn irq_default),
    vector!("USART2", fn irq_default),
    vector!("USART3", fn irq_default),
    vector!("EXTI15_10", fn irq_default),
    vector!("RTC_Alarm", fn irq_default),
    vector!("OTG_FS_WKUP", fn irq_default),
    vector!("TIM8_BRK_TIM12", fn irq_default),
    vector!("TIM8_UP_TIM13", fn irq_default),
    vector!("TIM8_TRG_COM_TIM14", fn irq_default),
    vector!("TIM8_CC", fn irq_default),
    vector!("DMA1_Stream7", fn irq_default),
    vector!("FSMC", fn irq_default),
    vector!("SDIO", fn irq_default),
    vector!("TIM5", fn irq_default),
    vector!("SPI3", fn irq_default),
    vector!("UART4", fn irq_default),
    vector!("UART5", fn irq_default),
    vector!("TIM6_DAC", fn irq_default),
    vector!("TIM7", fn irq_default),
    vector!("DMA2_Stream0", fn irq_default),
    vector!("DMA2_Stream1", fn irq_default),
    vector!("DMA2_Stream2", fn irq_default),
    vector!("DMA2_Stream3", fn irq_default),
    vector!("DMA2_Stream4", fn irq_default),
    vector!("ETH", fn irq_default),
    vector!("ETH_WKUP", fn irq_default),
    vector!("CAN2_TX", fn irq_default),
    vector!("CAN2_RX0", fn irq_default),
    vector!("CAN2_RX1", fn irq_default),
    vector!("CAN2_SCE", fn irq_default),
    vector!("OTG_FS", fn irq_default),
    vector!("DMA2_Stream5", fn irq_default),
    vector!("DMA2_Stream6", fn irq_default),
    vector!("DMA2_Stream7", fn irq_default),
    vector!("USART6", fn irq_default),
    vector!("I2C3_EV", fn irq_default),
    vector!("I2C3_ER", fn irq_default),
    vector!("OTG_HS_EP1_OUT", fn irq_default),
    vector!("OTG_HS_EP1_IN", fn irq_default),
    vector!("OTG_HS_WKUP", fn irq_default),
    vector!("OTG_HS", fn irq_default),
    vector!("DCMI", fn irq_default),
    vector!("CRYP", fn irq_default),
    vector!("HASH_RNG", fn irq_default),
    vector!("FPU", fn irq_default),
    vector!("UART7", fn irq_default),
    vector!("UART8", fn irq_default),
    vector!("SPI4", fn irq_default),
    vector!("SPI5", fn irq_default),
    vector!("SPI6", fn irq_default),
    vector!("SAI1", fn irq_default),
    vector!("LCD-TFT1", fn irq_default),
    vector!("LCD-TFT2", fn irq_default),
    vector!("DMA2D", fn irq_default)
);

// Cortex-M4 Vector Table we are going to bake into the binary
#[link_section = ".vector_table"]
#[no_mangle]
static VECTOR_TABLE : VectorTable<*const (), 109> = ISR_TABLE.raw();

#[naked]
#[no_mangle]
pub unsafe fn _start() {
    naked_asm!(
        "ldr     r0, =LD_STACK_PTR",
        "mov     sp, r0",
        "bl      main",
        ".equ PSCI_SYSTEM_OFF, 0x84000008",
        "ldr     r0, =PSCI_SYSTEM_OFF",
        "hvc     #0"
    );
}


#[no_mangle]
pub fn main() {
    write_slice(0x01 as _, b"Welcome");
}

#[no_mangle]
pub fn irq_default() -> ! {
    loop {}
}

