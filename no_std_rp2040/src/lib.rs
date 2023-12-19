#![no_main]
#![allow(unsafe_code)]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

use core::str::FromStr;

use edn_rs::{Edn, EdnError};
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn init_allocator() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

fn edn_from_str() -> Result<Edn, EdnError> {
    let edn_str = "{:a \"2\"   :b [true false] :c #{:A nil {:a :b}}}";
    Edn::from_str(edn_str)
}

fn main() {
    init_allocator();

    let _edn = edn_from_str();
}
