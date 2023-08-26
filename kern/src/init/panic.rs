use core::panic::PanicInfo;
use core::fmt::{Write, self};
use pi::uart::MiniUart;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = MiniUart::new();
    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
        uart.write_str(r#"
                 (
          (      )    )
           )    (    (
          (          `
      .-""^"""^""^"""^""-.
    (//\\//\\//\\//\\//\\//)
    ~\^^^^^^^^^^^^^^^^^^/~
      `================`

      The pi is overdone.

---------- PANIC ----------"#).ok();
    writeln!(uart, "").ok();
    if let Some(location) = info.location() {
        writeln!(uart, "FILE: {}", location.file()).ok();
        writeln!(uart, "LINE: {}", location.line()).ok();
        writeln!(uart, "COL: {}", location.column()).ok();
        writeln!(uart, "").ok();
    }

    if let Some(message) = info.message() {
        writeln!(uart, "{}", message).ok();
    }
    loop {}
}