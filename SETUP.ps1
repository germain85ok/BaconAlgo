# BaconAlgo 2030 - Setup Script
# Run this script to auto-install all dependencies and configure the project

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   BaconAlgo 2030 - Auto Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check Node.js
Write-Host "[1/5] Checking Node.js..." -ForegroundColor Yellow
if (Get-Command node -ErrorAction SilentlyContinue) {
    $nodeVersion = node --version
    Write-Host "✓ Node.js installed: $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Node.js not found. Please install Node.js 18+ from https://nodejs.org" -ForegroundColor Red
    exit 1
}

# Check Python
Write-Host "[2/5] Checking Python..." -ForegroundColor Yellow
if (Get-Command python -ErrorAction SilentlyContinue) {
    $pythonVersion = python --version
    Write-Host "✓ Python installed: $pythonVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Python not found. Please install Python 3.10+ from https://python.org" -ForegroundColor Red
    exit 1
}

# Install Frontend Dependencies
Write-Host "[3/5] Installing Frontend dependencies..." -ForegroundColor Yellow
Set-Location station
npm install
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Frontend dependencies installed" -ForegroundColor Green
} else {
    Write-Host "✗ Failed to install frontend dependencies" -ForegroundColor Red
    exit 1
}
Set-Location ..

# Install API Dependencies
Write-Host "[4/5] Installing API dependencies..." -ForegroundColor Yellow
Set-Location api
python -m pip install -r requirements.txt
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ API dependencies installed" -ForegroundColor Green
} else {
    Write-Host "✗ Failed to install API dependencies" -ForegroundColor Red
    exit 1
}
Set-Location ..

# Install Discord Bot Dependencies
Write-Host "[5/5] Installing Discord Bot dependencies..." -ForegroundColor Yellow
Set-Location discord-bot
python -m pip install -r requirements.txt
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Discord Bot dependencies installed" -ForegroundColor Green
} else {
    Write-Host "✗ Failed to install Discord Bot dependencies" -ForegroundColor Red
    exit 1
}
Set-Location ..

# Create .env files if they don't exist
Write-Host ""
Write-Host "Creating environment files..." -ForegroundColor Yellow

if (-not (Test-Path "station/.env")) {
    Copy-Item "station/.env.example" "station/.env" -ErrorAction SilentlyContinue
    Write-Host "✓ Created station/.env (please configure)" -ForegroundColor Green
}

if (-not (Test-Path "discord-bot/.env")) {
    Copy-Item "discord-bot/.env.example" "discord-bot/.env" -ErrorAction SilentlyContinue
    Write-Host "✓ Created discord-bot/.env (please configure)" -ForegroundColor Green
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   Setup Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Configure station/.env with your Supabase credentials" -ForegroundColor White
Write-Host "2. Configure discord-bot/.env with your Discord bot token (optional)" -ForegroundColor White
Write-Host "3. Run START_BACONALGO.bat to start all services" -ForegroundColor White
Write-Host ""
Write-Host "For detailed instructions, see QUICKSTART.md" -ForegroundColor White
Write-Host ""

Pause
