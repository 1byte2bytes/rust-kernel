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
    print!("\nSweetie Kernel - Copyright Sydney Erickson 2016");
    print!("\n-----------------------------------------------\n");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    print!("memory areas:\n");
    let mut total_memory = 0x0;
    for area in memory_map_tag.memory_areas() {
        print!("    start: 0x{:x}, length: 0x{:x}\n",
            area.base_addr, area.length);
            total_memory += area.length;
    }
    print!("Total memory: 0x{:x}\n\n", total_memory);

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    /*print!("kernel sections:\n");
    for section in elf_sections_tag.sections() {
        print!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}\n",
            section.addr, section.size, section.flags);
    }*/

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    print!("Kernel start: 0x{:x}, end: 0x{:x}\n", kernel_start, kernel_end);

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    print!("Multiboot start: 0x{:x}, end: 0x{:x}\n", multiboot_start, multiboot_end);

    panic!();

    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> !
{
    print!("\n\nPANIC in {} at line {}:", file, line);
    print!("    {}", fmt);
    loop{}
}