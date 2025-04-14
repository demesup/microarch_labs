@echo off
setlocal

:: Check if LAB_NUM environment variable is set
if "%LAB_NUM%"=="" (
    echo Error: LAB_NUM environment variable is not set.
    echo Please run set_lab_env.bat first to set the lab number.
    exit /b 1
)

:: Read optional binary name
set "APP_NAME=%1"
if "%APP_NAME%"=="" set "APP_NAME=main"

:: Define the path to the lab's main folder
set "LAB_PATH=E:\rust\lab%LAB_NUM%\main"

:: Ensure the path exists
if not exist "%LAB_PATH%" (
    echo Error: Path "%LAB_PATH%" does not exist.
    exit /b 1
)

:: Navigate to the project folder
cd /d "%LAB_PATH%"
echo Listing contents of %LAB_PATH%:

:: Build the project
echo Building lab%LAB_NUM%...
cargo build

:: List the target directory to check build output
echo Listing target directory:

:: Check if the build succeeded
if errorlevel 1 (
    echo Build failed.
    exit /b 1
)

:: Construct full path to the binary
set "BINARY_PATH=%LAB_PATH%\target\thumbv8m.main-none-eabihf\debug\%APP_NAME%"

:: Check if the binary exists
if not exist "%BINARY_PATH%" (
    echo Error: Binary "%BINARY_PATH%" not found.
    exit /b 1
)

:: Run with probe-rs
echo.
echo Running: probe-rs run --chip RP235x "%BINARY_PATH%"
probe-rs run --chip RP235x "%BINARY_PATH%"
