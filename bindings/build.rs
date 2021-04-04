fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{AllocConsole, AttachConsole},
    );
}
