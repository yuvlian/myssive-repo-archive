$originalDirectory = Get-Location

if (-not (Get-Command mdbook -ErrorAction SilentlyContinue)) {
    Write-Host "mdbook not found. Installing..."
#https://github.com/sunface/rust-by-practice.git
    Set-ExecutionPolicy Unrestricted -Scope Process
    iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content

    cargo-binstall mdbook
}

$rustPracticePath = Join-Path $originalDirectory "rust-by-practice"

if (Test-Path $rustPracticePath) {
    Set-Location $rustPracticePath

    Start-Process -FilePath mdbook -ArgumentList "serve", "en/"
    
    Set-Location $originalDirectory

    Start-Process "http://localhost:3000"
} else {
    Write-Host "rust-by-practice directory not found."
}
