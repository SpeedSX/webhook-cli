@echo off
echo Building Webhook CLI...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ Build successful!
    echo.
    echo The CLI is available at: target\release\webhook.exe
    echo.
    echo To test it, run:
    echo   .\target\release\webhook.exe generate
    echo.
    echo To install globally, copy the exe to a folder in your PATH
    echo or run: cargo install --path .
) else (
    echo.
    echo ❌ Build failed!
    echo Make sure Rust is installed: https://rustup.rs/
)

pause
