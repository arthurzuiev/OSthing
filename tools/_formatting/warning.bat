@echo off

set warningText=%1

echo:
echo:

powershell -Command "Write-Host \"*************************************************************************************\" -ForegroundColor DarkYellow
powershell -Command "Write-Host \"[TOOL][WARNING] %warningText%\" -ForegroundColor DarkYellow
powershell -Command "Write-Host \"*************************************************************************************\" -ForegroundColor DarkYellow
