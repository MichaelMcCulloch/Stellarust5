
$workingDir  = Get-Location
$scriptDir ="$workingDir\.vscode\"

Start-Job -FilePath "$scriptDir\watch_frontend.ps1"  -ArgumentList "$workingDir"
Start-Job -FilePath "$scriptDir\watch_backend.ps1"  -ArgumentList "$workingDir"
Start-Job -FilePath "$scriptDir\watch_trigger.ps1"  -ArgumentList "$workingDir"


While (Get-Job -State "Running") { Start-Sleep 1 }

# Display output from all jobs
Get-Job | Receive-Job

# Cleanup
Remove-Job *