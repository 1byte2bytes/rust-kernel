#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
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

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    print!("memory areas:\n");
    for area in memory_map_tag.memory_areas() {
        print!("    start: 0x{:x}, length: 0x{:x}\n",
            area.base_addr, area.length);
    }

    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}