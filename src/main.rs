#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable Rust's normal entry point handling
#![feature(abi_x86_interrupt)] // Enable x86 interrupt ABI

mod vga_buffer;
use core::panic::PanicInfo;
use core::arch::asm;

#[unsafe(no_mangle)] // Ensure the symbol name is preserved
pub extern "C" fn _start() -> ! {
    clear_screen(0x0f); // Clear screen with white-on-black color
   
    vga_buffer::print_something();
    loop {
        halt(); // Halt the CPU to save power
    }
}

const VGA_WIDTH: usize = 80;  // Columns in VGA text mode
const VGA_HEIGHT: usize = 25; // Rows in VGA text mode
static mut CURSOR_POS: usize = 0; // Current cursor position

fn clear_screen(color: u8) {
    let vga = unsafe { &mut *(0xb8000 as *mut [[u8; 2]; VGA_WIDTH * VGA_HEIGHT]) };
    for row in vga.iter_mut() {
        row[0] = b' '; // Fill with space character
        row[1] = color; // Set color attribute
    }
    unsafe { CURSOR_POS = 0 }; // Reset cursor position
}

fn print(s: &str) {
    unsafe {
        let vga_buffer = 0xb8000 as *mut u8;
        for (i, &byte) in s.as_bytes().iter().enumerate() {
            let pos = CURSOR_POS + i;
            if pos >= VGA_WIDTH * VGA_HEIGHT {
                scroll_screen(); // Scroll if we reach the bottom
                CURSOR_POS -= VGA_WIDTH;
            }
            *vga_buffer.add(pos * 2) = byte; // Write character
            *vga_buffer.add(pos * 2 + 1) = 0x0f; // White-on-black color
        }
        CURSOR_POS += s.len(); // Update cursor position
    }
}

fn print_ln(s: &str) {
    print(s); // Print the string
    unsafe {
        CURSOR_POS = (CURSOR_POS / VGA_WIDTH + 1) * VGA_WIDTH; // Move to next line
        if CURSOR_POS >= VGA_WIDTH * VGA_HEIGHT {
            scroll_screen(); // Scroll if we reach the bottom
            CURSOR_POS -= VGA_WIDTH;
        }
    }
}

fn scroll_screen() {
    let vga = unsafe { &mut *(0xb8000 as *mut [[u8; 2]; VGA_WIDTH * VGA_HEIGHT]) };
    for i in 0..(VGA_WIDTH * (VGA_HEIGHT - 1)) {
        vga[i] = vga[i + VGA_WIDTH]; // Move each line up
    }
    for i in (VGA_WIDTH * (VGA_HEIGHT - 1))..(VGA_WIDTH * VGA_HEIGHT) {
        vga[i] = [b' ', 0x0f]; // Clear the bottom line
    }
}

fn halt() {
    unsafe {
        asm!("hlt", options(nomem, nostack)); // Use inline assembly to halt
    }
}

fn u32_to_str(mut num: u32, buffer: &mut [u8]) -> &[u8] {
    let mut i = 0;
    if num == 0 {
        buffer[0] = b'0'; // Handle zero case
        return &buffer[0..1];
    }
    
    while num > 0 && i < buffer.len() {
        buffer[i] = b'0' + (num % 10) as u8; // Convert digit to ASCII
        num /= 10;
        i += 1;
    }
    &buffer[0..i] // Return the slice containing the number
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_ln("KERNEL PANIC!");
    if let Some(loc) = info.location() {
        print_ln("At:");
        print_ln(loc.file()); // Print file name
        
        // Print line number
        let mut buf = [0u8; 10];
        let line_str = u32_to_str(loc.line(), &mut buf);
        print("Line: ");
        print(unsafe { core::str::from_utf8_unchecked(line_str) });
        print_ln("");
    }
    loop {
        halt(); // Halt the CPU on panic
    }
}
