#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

use std::panic::{self, PanicInfo};

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    console::attach();

    panic::set_hook(Box::new(|e| {
        console::attach();
        panic_hook(e);
    }));

    println!("Hello, world !");
}

fn panic_hook(info: &PanicInfo<'_>) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "???",
        },
    };
    let location = info.location().unwrap();

    eprintln!("Panic : '{}', {}", msg, location);
}

mod console {
    use bindings::Windows::Win32::SystemServices::{AllocConsole, AttachConsole};

    #[allow(clippy::missing_safety_doc)]
    pub fn attach() {
        unsafe {
            // u32::MAX = DWORD(-1) soit ATTACH_PARENT_PROCESS
            if !AttachConsole(u32::MAX).as_bool() {
                AllocConsole();
            }
        }
    }
}
