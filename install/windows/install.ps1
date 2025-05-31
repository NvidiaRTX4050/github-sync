# Requires -RunAsAdministrator

$ServiceName = "GitHubSync"
$DisplayName = "GitHub Sync Service"
$Description = "Two-way file synchronization using Git"
$BinaryPath = Join-Path $env:ProgramFiles "GitHubSync\github-sync.exe"
$InstallDir = Join-Path $env:ProgramFiles "GitHubSync"

# Check if running as administrator
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Error "Please run this script as Administrator"
    exit 1
}

# Create installation directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# Copy binary
Copy-Item "github-sync.exe" -Destination $BinaryPath -Force

# Create and start the service
$service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($service) {
    Write-Host "Service already exists. Stopping and removing..."
    Stop-Service -Name $ServiceName -Force
    Start-Sleep -Seconds 2
    Remove-Service -Name $ServiceName
}

Write-Host "Creating service..."
New-Service -Name $ServiceName `
    -DisplayName $DisplayName `
    -Description $Description `
    -BinaryPathName $BinaryPath `
    -StartupType Automatic

# Start the service
Write-Host "Starting service..."
Start-Service -Name $ServiceName

# Add firewall rule
$firewallRule = Get-NetFirewallRule -DisplayName $ServiceName -ErrorAction SilentlyContinue
if (-not $firewallRule) {
    New-NetFirewallRule -DisplayName $ServiceName `
        -Direction Inbound `
        -Action Allow `
        -Program $BinaryPath `
        -Description "Allow GitHub Sync Service"
}

Write-Host "✅ GitHub Sync service installed successfully"
Write-Host "Service status:"
Get-Service -Name $ServiceName | Format-List 