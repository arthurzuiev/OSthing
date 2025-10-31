# === Path stuff ===
#$toolPath = Split-Path -Parent $MyInvocation.MyCommand.Definition

# === Check for admin rights ===
$isAdmin = ([bool](net session 2>$null))
if (-not $isAdmin) {
    Clear-Host
    Write-Host "************************************************************************************" -ForegroundColor DarkYellow
    Write-Host "[TOOL][WARNING] No admin rights were given. Because of that AutoInstall is disabled!" -ForegroundColor DarkYellow
    Write-Host "************************************************************************************" -ForegroundColor DarkYellow
    Write-Host "Continuing check..." -ForegroundColor DarkMagenta
    Pause
    Clear-Host
}

# === Check if Chocolatey exists and we have admin rights ===
$chocoCheck = & choco -v 2>$null

if (-not $chocoCheck) {
    Clear-Host
    Write-Host "****************************************************************************************" -ForegroundColor DarkYellow
    Write-Host "Chocolatey is not installed! You can install it from https://chocolatey.org/install" -ForegroundColor DarkYellow
    Write-Host "Because of that AutoInstall is disabled!" -ForegroundColor DarkYellow
    Write-Host "****************************************************************************************" -ForegroundColor DarkYellow
    Write-Host ""
    Write-Host "Tip: You can install it quickly by running this command as Administrator:" -ForegroundColor DarkGray
    Write-Host 'Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString("https://community.chocolatey.org/install.ps1"))' -ForegroundColor DarkGray
    Write-Host "Continuing check..." -ForegroundColor DarkMagenta
    Pause
    Clear-Host
}

$autoInstall = ($chocoCheck -and $isAdmin)

# === Dependency Lists ===
$deps = @(
    @{Name='Rust'; Check='rustc --version'; Install='choco install rust -y'},
    @{Name='Cargo'; Check='cargo --version'; Install='choco install rust -y'},
    @{Name='mkisofs'; Check='mkisofs -version'; Install='choco install cdrtools -y'},
    @{Name='QEMU'; Check='qemu-system-x86_64 --version'; Install='choco install qemu -y'},
    @{ 
        Name='MSYS2 | (Sorry for console jumpscare :D)'; 
        Check='msys2 -v'; 
        Install='choco install msys2 -y' 
    }
)

Write-Host "=== Dependency Check ===`n" -ForegroundColor DarkGray

foreach ($dep in $deps) {
    try {
        & cmd /c $dep.Check >$null 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[OK] $($dep.Name)" -ForegroundColor Green
        } else {
            if ($autoInstall -and $dep.Install) {
                Write-Host "[INSTALLING] $($dep.Name)..." -ForegroundColor Yellow
                # Run install, capture output, print in DarkGray
                $installOutput = & cmd /c $dep.Install 2>&1
                $installOutput | ForEach-Object { Write-Host $_ -ForegroundColor DarkGray }
                Write-Host "[INSTALLED] $($dep.Name)" -ForegroundColor Magenta
            } else {
                Write-Host "[MISSING] $($dep.Name)" -ForegroundColor Red
                if ($dep.Install) {
                    Write-Host "You can install it with: $($dep.Install)"
                }
            }
        }
    } catch {
        Write-Host "[ERROR] $($dep.Name)" -ForegroundColor Red
    }
}

Write-Host "`n=== Done ==="  -ForegroundColor DarkGray
Pause
