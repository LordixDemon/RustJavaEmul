@echo off
title Age of Heroes I - RustJava
set RUSTJAVA_SCREEN=240x320
cd /d "%~dp0"
echo Starting Age of Heroes I (240x320)...
"%~dp0target\release\rust_java.exe" -jar "%~dp0apk\spaces-java\all-games\23561679-Age_of_Heroes1_240.jar"
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Emulator exited with error code %ERRORLEVEL%.
    pause
)
