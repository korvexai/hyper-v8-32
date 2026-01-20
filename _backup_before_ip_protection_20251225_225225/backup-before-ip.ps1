# ================= BACKUP BEFORE IP PROTECTION =================

$projectRoot = Get-Location
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$backupDir = Join-Path $projectRoot "_backup_before_ip_protection_$timestamp"

Write-Host "Creating backup folder..."
New-Item -ItemType Directory -Path $backupDir -Force | Out-Null

Write-Host "Copying project files..."
Copy-Item -Path "$projectRoot\*" -Destination $backupDir `
    -Recurse -Force `
    -Exclude "_backup_before_ip_protection_*", ".git"

Write-Host "Backup created in:"
Write-Host $backupDir
