use std::sync::OnceLock;

static NO_COLOR: OnceLock<bool> = OnceLock::new();

pub fn init(no_color: bool) {
    // Ignore if already initialized; first value wins.
    let _ = NO_COLOR.set(no_color);

    if no_color {
        colored::control::set_override(false);
    } else {
        // Enable ANSI support on Windows
        #[cfg(windows)]
        enable_ansi_support();
        
        // On Unix systems, colored crate handles this automatically
    }
}

// Allow other modules to check whether colors should be emitted.
pub fn is_color_enabled() -> bool {
    !*NO_COLOR.get().unwrap_or(&false)
}

#[cfg(windows)]
fn enable_ansi_support() {
    use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, SetConsoleMode, GetStdHandle, 
        STD_OUTPUT_HANDLE, ENABLE_VIRTUAL_TERMINAL_PROCESSING
    };
    
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
