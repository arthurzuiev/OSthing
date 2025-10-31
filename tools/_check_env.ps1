# Filename: Env-Checker.ps1
# Purpose: Check Rust/Cargo environment, fix automatically if possible

# === Color helpers ===
function Write-Info($msg) { Write-Host $msg -ForegroundColor DarkGray }
function Write-OK($msg) { Write-Host $msg -ForegroundColor Green }
function Write-Warn($msg) { Write-Host $msg -ForegroundColor Yellow }
function Write-ErrorMsg($msg) { Write-Host $msg -ForegroundColor Red }
function Write-Action($msg) { Write-Host $msg -ForegroundColor Magenta }

# === Environment dictionary ===
$envChecks = @{
    "Rust Src Component" = @{
        check = { rustup component list | Select-String "rust-src.*installed" }
        fix   = { rustup component add rust-src }
    }
    "LLVM Tools" = @{
        check = { rustup component list | Select-String "llvm-tools.*(default|installed)" }
        fix   = { rustup component add llvm-tools }
    }
    "Bootimage" = @{
        check = { cargo bootimage --version 2>$null; return ($LASTEXITCODE -eq 0) }
        fix   = { cargo install bootimage }
    }
    "Nightly Installed" = @{
        check = { rustup toolchain list | Select-String "nightly" }
        fix   = { rustup toolchain install nightly }
    }
    "Nightly Default" = @{
        check = { 
            $toolchain = rustup show active-toolchain 2>$null
            return $toolchain -match "nightly"
        }
        fix   = { rustup default nightly }
    }
}

Write-Info "`n=== Environment Check ==="

# === Loop through checks ===
foreach ($key in $envChecks.Keys) {
    $result = & $envChecks[$key].check
    if ($result) {
        Write-OK "[OK] $key"
    } else {
        Write-Warn "[MISSING] $key"
        if ($envChecks[$key].fix) {
            Write-Action "[FIXING] $key..."
            & $envChecks[$key].fix
            # re-check after fix
            $recheck = & $envChecks[$key].check
            if ($recheck) { Write-OK "[FIXED] $key" } 
            else { Write-ErrorMsg "[FAILED TO FIX] $key" }
        } else {
            Write-ErrorMsg "[NO FIX AVAILABLE] $key"
        }
    }
}

Write-Info "`n=== Environment Check Complete ==="
