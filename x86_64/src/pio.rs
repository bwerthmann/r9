pub unsafe fn outb(_port: u16, _b: u8) {
    #[cfg(not(test))]
    unsafe {
        core::arch::asm!("outb %al, %dx", in("dx") port, in("al") b, options(att_syntax));
    }
}

pub unsafe fn outw(_port: u16, _w: u16) {
    #[cfg(not(test))]
    unsafe {
        core::arch::asm!("outw %ax, %dx", in("dx") port, in("ax") w, options(att_syntax));
    }
}

pub unsafe fn outl(_port: u16, _l: u32) {
    #[cfg(not(test))]
    unsafe {
        core::arch::asm!("outl %eax, %dx", in("dx") port, in("ax") l, options(att_syntax));
    }
}
