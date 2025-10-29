@echo off
REM Enable ANSI escape sequences
for /f "delims=" %%i in ('echo prompt $E ^| cmd') do set "ESC=%%i"

set errorText=%1

echo:
echo:
powershell -Command "Write-Host \"*************************************************************************************\" -ForegroundColor DarkRed
powershell -Command "Write-Host \"[TOOL][ERROR] %errorText%\" -ForegroundColor DarkRed
powershell -Command "Write-Host \"*************************************************************************************\" -ForegroundColor DarkRed

