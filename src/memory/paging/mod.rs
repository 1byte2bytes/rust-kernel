use memory::PAGE_SIZE;

const ENTRY_COUNT: usize = 512;

pub type PhyscialAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize,
}

pub fn translate(virtual_address: VirtualAddress) -> Option<PhyscialAddress> {
    let offset = virtual_address % PAGE_SIZE;
    translate_page(Page::containing_address(virtual_address))
        .map(|frame| frame.number * PAGE_SIZE + offset)
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 ||
            address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}", address);
        Page { number: address / PAGE_SIZE }
    }
}