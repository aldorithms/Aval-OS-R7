// kernel entry to the system
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

//panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)
    -> ! 
    {
        loop {}
    }

#[no_mangle]
pub extern "C" fn _start() 
    -> ! 
    {
        loop {}
    }


// Path: src\lib.rs