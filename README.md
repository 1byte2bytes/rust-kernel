#Sydney's Rust Kernel
This kernel isn't really useful for much. It probably won't even build correctly for you because I messed up the dependancies somehow. But if you tinker with it and get it working or I fix it eventually it does work!

##What is this kernel?
It's a minimal 64 bit kernel made in Rust. Right now it uses Assembly to create the stack and switch from 16 bits to 32 bits to 64 bits. It also has a basic video (console) driver, frame allocation system, and soon to have paging tables.

##What's on the todo list?
- More comments in the rust sections of the code
- Page tables
- Remapping the kernel
- Kernel heaps
- Catching exceptions
- Implementing better exception messages
- Returning from exceptions for continued operation
- Double faults 
- Setting up basic I/O
- Configuring the PIC
- Implementing a keyboard driver
- Implementing a serial driver
- Basic POSIX support(?)

###Resources used
http://os.phil-opp.com/
http://www.randomhacks.net/bare-metal-rust/
http://wiki.osdev.org/Main_Page

###Useful links
https://www.rust-lang.org/en-US/install.html
http://www.nasm.us/
https://crates.io/
https://www.gnu.org/software/grub/