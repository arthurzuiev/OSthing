@echo off
set errorText=%1

echo:
echo:
powershell -Command "Write-Host \"*************************************************************************************\" -ForegroundColor DarkRed
powershell -Command "Write-Host \"[TOOL][ERROR] %errorText%\" -ForegroundColor DarkRed
powershell -Command "Write-Host \"*************************************************************************************\" -ForegroundColor DarkRed

