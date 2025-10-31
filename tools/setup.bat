@echo off
:: Directory of the current batch file
set "toolsfolder=%~dp0"

:: run checks
cls
powershell %toolsfolder%\_check_deps.ps1
cls
powershell %toolsfolder%\_check_env.ps1