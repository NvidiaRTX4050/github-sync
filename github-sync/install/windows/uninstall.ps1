# Requires -RunAsAdministrator

$ServiceName = "GitHubSync"
$InstallDir = Join-Path $env:ProgramFiles "GitHubSync"

# Check if running as administrator
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Error "Please run this script as Administrator"
    exit 1
}

# Stop and remove service
$service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($service) {
    Write-Host "Stopping service..."
    Stop-Service -Name $ServiceName -Force
    Start-Sleep -Seconds 2
    Write-Host "Removing service..."
    Remove-Service -Name $ServiceName
}

# Remove firewall rule
$firewallRule = Get-NetFirewallRule -DisplayName $ServiceName -ErrorAction SilentlyContinue
if ($firewallRule) {
    Write-Host "Removing firewall rule..."
    Remove-NetFirewallRule -DisplayName $ServiceName
}

# Remove installation directory
if (Test-Path $InstallDir) {
    Write-Host "Removing installation files..."
    Remove-Item -Path $InstallDir -Recurse -Force
}

Write-Host "✅ GitHub Sync service uninstalled successfully" 