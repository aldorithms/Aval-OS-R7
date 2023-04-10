#![no_std]
#![no_main]
#![feature(custom_test_frameworks, panic_info_message)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

//panic handler for testing
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) 
    -> ! 
    {
        test_panic_handler(info)
    }

//test_main function
#[cfg(test)]
pub fn test_main() 
    {
        println!("Running tests...");
        test_runner();
        println!("All tests passed!");
    }

#[no_mangle]
pub extern "C" fn _start() 
    -> ! 
    {
        #[cfg(test)]
        test_main();

        loop {}
    }


// Path: test\lib.rs