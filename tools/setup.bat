@echo off
:: Directory of the current batch file
set "toolsfolder=%~dp0"

:: Get the Python executable that 'py' would run
for /f "delims=" %%i in ('py -c "import sys; print(sys.executable)"') do set "python=%%i"

:: Install dependencies
"%python%" -m pip install -r "%toolsfolder%requirements.txt"

:: Run scripts
"%python%" "%toolsfolder%dep_check.py"
"%python%" "%toolsfolder%env_check.py"
