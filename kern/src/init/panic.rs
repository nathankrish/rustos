use core::panic::PanicInfo;
use crate::console::kprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kprintln!("{}", "---------- PANIC ----------");
    if let Some(location) = _info.location() {
        kprintln!("FILE: {}", location.file());
        kprintln!("LINE: {}", location.line());
        kprintln!("COL: {}", location.column());
    }
    if let Some(msg) = _info.message() {
        kprintln!("{}", msg);
    }
    loop {}
}
