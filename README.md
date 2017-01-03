#Sydney's Rust Kernel
This kernel isn't really useful for much. It probably won't even build correctly for you because I messed up the dependancies somehow. But if you tinker with it and get it working or I fix it eventually it does work!

##What is this kernel?
It's a minimal 64 bit kernel made in Rust. Right now it uses Assembly to create the stack and switch from 16 bits to 32 bits to 64 bits. It also has a basic video (console) driver, frame allocation system, and soon to have paging tables.