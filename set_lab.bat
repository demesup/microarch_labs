@echo off
setlocal

:: Check if the lab number is provided
if "%~1"=="" (
    echo Usage: set_lab_env.bat [lab_number]
    exit /b 1
)

:: Set the lab number environment variable
set "LAB_NUM=%1"

:: Inform the user
echo Lab number set to %LAB_NUM%.
echo You can now run other batch files using this lab number.

:: Keep the command window open for further commands
cmd /k
