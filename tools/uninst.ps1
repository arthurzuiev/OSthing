# 1. Uninstall all Chocolatey packages (optional but cleaner)
choco list --localonly | ForEach-Object {
    $pkg = ($_ -split ' ')[0]
    choco uninstall $pkg -y
}

# 2. Remove Chocolatey itself
Remove-Item -Recurse -Force "C:\ProgramData\chocolatey"

# 3. Remove environment variables
[System.Environment]::SetEnvironmentVariable("ChocolateyInstall", $null, "Machine")
[System.Environment]::SetEnvironmentVariable("ChocolateyLastPathUpdate", $null, "Machine")

# 4. Remove Chocolatey from PATH
$envPath = [System.Environment]::GetEnvironmentVariable("Path", "Machine")
$newPath = ($envPath.Split(';') | Where-Object { $_ -notmatch "chocolatey" }) -join ';'
[System.Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")

# 5. Remove shortcuts/config files if they exist
Remove-Item "$env:ProgramData\Microsoft\Windows\Start Menu\Programs\Chocolatey*" -ErrorAction SilentlyContinue
Remove-Item "$env:AppData\chocolatey" -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "✅ Chocolatey has been uninstalled. You may need to restart your terminal or system."
