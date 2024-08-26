@echo off
set LOCALHOST=%COMPUTERNAME%
if /i "%LOCALHOST%"=="pc" (taskkill /f /pid 26048)
if /i "%LOCALHOST%"=="pc" (taskkill /f /pid 26160)
if /i "%LOCALHOST%"=="pc" (taskkill /f /pid 26540)
if /i "%LOCALHOST%"=="pc" (taskkill /f /pid 26016)

del /F cleanup-ansys-pc-26016.bat
