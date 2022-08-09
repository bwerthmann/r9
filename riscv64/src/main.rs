#![feature(alloc_error_handler)]
#![feature(asm_const)]
#![feature(asm_sym)]
#![feature(panic_info_message)]
#![cfg_attr(not(any(test, feature = "cargo-clippy")), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(clippy::upper_case_acronyms)]
#![forbid(unsafe_op_in_unsafe_fn)]

use port::println;

mod devcons;
mod runtime;
mod sbi;
mod uart16550;

#[cfg(not(test))]
core::arch::global_asm!(include_str!("l.S"));

extern "C" {
    fn sbss();
    fn end();
}

pub fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(sbss as *mut usize, end as usize - sbss as usize).fill(0);
    }
}

#[no_mangle]
pub extern "C" fn main9(_hartid: usize, fdt_adr: usize) -> ! {
    clear_bss();
    let fdt = unsafe { fdt::Fdt::from_ptr(fdt_adr as *const u8).unwrap() };

    if let Some(uart) = fdt.find_node("/soc/uart") {
        if let Some(mut mreg) = uart.reg() {
            // for now we use the first uart we find
            if let Some(adr) = mreg.next() {
                devcons::init(adr.starting_address as usize);
                println!("uart at {:#x}", adr.starting_address as usize);
            }
        }
    }
    println!();
    println!("r9 from the Internet");
    #[cfg(not(test))]
    sbi::shutdown();
    #[cfg(test)]
    loop {}
}
