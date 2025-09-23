use std::sync::OnceLock;

static NO_COLOR: OnceLock<bool> = OnceLock::new();

pub fn init(no_color: bool) {
    NO_COLOR.set(no_color).unwrap_or({});
    
    if no_color {
        colored::control::set_override(false);
    } else {
        // Enable ANSI support on Windows
        #[cfg(windows)]
        enable_ansi_support();
        
        // On Unix systems, colored crate handles this automatically
    }
}

#[cfg(windows)]
fn enable_ansi_support() {
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    
    unsafe {
        let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if stdout_handle != INVALID_HANDLE_VALUE {
            let mut mode: u32 = 0;
            if GetConsoleMode(stdout_handle, &mut mode) != 0 {
                let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
                SetConsoleMode(stdout_handle, new_mode);
            }
        }
    }
}
