@echo off
echo ==========================================
echo       DANG BUILD DU AN UGGO-DETECTER
echo ==========================================
echo.
echo 1. Dang kiem tra moi truong Rust...
cargo --version
if %errorlevel% neq 0 (
    echo [LOI] Ban chua cai Rust! Hay cai dat Rust tu https://rustup.rs
    pause
    exit /b
)

echo.
echo 2. Dang tai thu vien va bien dich (se mat vai phut)...
cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo [LOI] Qua trinh Build THAT BAI. Hay kiem tra lai code.
    pause
    exit /b
)

echo.
echo ==========================================
echo        BUILD THANH CONG! (SUCCESS)
echo ==========================================
echo File chay cua ban nam tai: target\release\uggo.exe
echo.
echo Dang mo thu muc chua file...
explorer "target\release"
pause
