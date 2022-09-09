#![feature(
    alloc_error_handler,
    asm_const,
    asm_sym,
    fn_align,
    naked_functions,
    panic_info_message,
    stmt_expr_attributes
)]
#![cfg_attr(not(any(test, feature = "cargo-clippy")), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(clippy::upper_case_acronyms)]
#![forbid(unsafe_op_in_unsafe_fn)]

use port::println;

mod devcons;
mod runtime;
mod sbi;
mod sbiuart;

#[cfg(not(test))]
core::arch::global_asm!(include_str!("l.S"));

#[no_mangle]
pub extern "C" fn main9(hartid: usize, fdt_adr: *const u8) -> ! {
    devcons::init();
    println!();
    println!("r9 from the Internet");
    println!("dtb at {:#x}", fdt_adr as usize);
    println!("hartid {}", hartid);
    #[cfg(not(test))]
    sbi::shutdown();
    #[cfg(test)]
    loop {}
}
