#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();
    print!("   _____           _  ____   _____ \n");
    print!("  / ____|         | |/ __ \\ / ____|\n");
    print!(" | (___  _   _  __| | |  | | (___  \n");
    print!("  \\___ \\| | | |/ _` | |  | |\\___ \\ \n");
    print!("  ____) | |_| | (_| | |__| |____) |\n");
    print!(" |_____/ \\__, |\\__,_|\\____/|_____/ \n");
    print!("          __/ |                    \n");
    print!("         |___/                     \n");
    print!("\nSydOS Sweetie - Copyright Sydney Erickson 2016");
    print!("\n----------------------------------------------\n");

    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}